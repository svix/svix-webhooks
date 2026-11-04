use sea_orm::{DbErr, RuntimeErr};

pub fn is_conflict_err(db_err: &DbErr) -> bool {
    if let DbErr::RecordNotInserted = db_err {
        return true;
    }

    // DbErr is `#[non_exhaustive]` as of sea-orm 2.0, so we can't enumerate the remaining
    // variants and get a compile time error when new ones show up. Only the variants below
    // can carry a driver error, everything else is a no.
    let (DbErr::Exec(rt_err) | DbErr::Query(rt_err) | DbErr::Conn(rt_err)) = db_err else {
        return false;
    };

    let sqlx_err = match rt_err {
        RuntimeErr::SqlxError(e) => e,
        RuntimeErr::Internal(_) => return false,
    };

    sqlx_err
        .as_database_error()
        .and_then(|e| e.code())
        .filter(|code| code == "23505")
        .is_some()
}

pub fn is_timeout_error(db_err: &DbErr) -> bool {
    let (DbErr::Conn(runtime_err) | DbErr::Exec(runtime_err) | DbErr::Query(runtime_err)) = &db_err
    else {
        return false;
    };

    let sqlx_err = match runtime_err {
        RuntimeErr::SqlxError(e) => e,
        RuntimeErr::Internal(_) => return false,
    };

    match sqlx_err.as_database_error() {
        // STUPID - no other good way to ID statement timeouts
        Some(e) => e
            .message()
            .contains("canceling statement due to statement timeout"),
        None => false,
    }
}

/// Returns true if the DbErr results from weirdness with a slow/long connection.
///
/// This is distinct from [is_timeout_error], which reports whether the underlying
/// query actually timed out on the pg side.
///
/// [is_connection_timeout_error] reports whether the connection to pg itself was slow
/// for some reason.
pub fn is_connection_timeout_error(db_err: &DbErr) -> bool {
    use DbErr as E;
    let rt_err = match db_err {
        E::ConnectionAcquire(_) | E::Conn(_) => return true,
        E::Exec(e) | E::Query(e) => e.to_string(),

        // DbErr is `#[non_exhaustive]` as of sea-orm 2.0, so we can't enumerate the
        // remaining variants and get a compile time error when new ones show up.
        // Only the variants above can carry a driver error, everything else is a no.
        _ => return false,
    };

    const ERRORS: [&str; 3] = [
        "Connection pool timed out",
        "Connection reset by peer",
        "unexpected end of file",
    ];
    for e in ERRORS {
        if rt_err.contains(e) {
            return true;
        }
    }

    false
}
