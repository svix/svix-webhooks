use crate::core::types::ApplicationIdOrUid;
use crate::error::{HttpError, ValidationErrorItem};
use crate::v1::utils::openapi_tag;
use crate::{ctx, err_generic, err_validation, AppState};
use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{DateTime, Timelike, Utc};
use schemars::JsonSchema;
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use svix_server_derive::aide_annotate;

/// Period length for a statistics data point
#[derive(Deserialize, Serialize, JsonSchema, Debug, Eq, PartialEq, Copy, Clone)]
#[allow(unused)]
pub enum StatisticsPeriod {
    OneDay,
    FiveMinutes,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BaseStatisticsResponse {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub period: StatisticsPeriod,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttemptStatisticsData {
    pub success_count: Option<Vec<i64>>,
    pub failure_count: Option<Vec<i64>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttemptStatisticsResponse {
    #[serde(flatten)]
    pub common_: BaseStatisticsResponse,
    pub data: AttemptStatisticsData,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct StatisticsQueryParams {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

/// Rows given by the app attempt stats query look like this.
#[derive(Debug, FromQueryResult)]
struct AppAttemptStatsResult {
    /// 0 = success, 2 = failure
    status: i16,
    count: i64,
    bucket: DateTime<Utc>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationPath {
    pub app_id: ApplicationIdOrUid,
}

/// Returns application-level statistics on message attempts
#[aide_annotate(op_id = "v1.stats.app-attempts")]
async fn get_app_attempt_stats(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationPath { app_id }): Path<ApplicationPath>,
    // FIXME: date parse fails give a 400 and oblique feedback
    Query(params): Query<StatisticsQueryParams>,
    // FIXME: any auth?
) -> Result<Json<AttemptStatisticsResponse>, crate::error::Error> {
    let (start_date, end_date) = validate_date_range(
        params.start_date,
        params.end_date,
        StatisticsPeriod::FiveMinutes,
        chrono::Duration::hours(6),
        chrono::Duration::days(1),
    )
    .map_err(|e| {
        HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec![String::from("query")],
            msg: e.to_string(),
            ty: "value_error".to_string(),
        }])
    })?;

    // RE: date bounds:
    // More simple to focus only on the ended_at since we know we only care about attempts
    // that already have an outcome.
    // If we need more sophisticated, we'd look for overlapping ranges of start/end vs created/ended.
    // N.b. `BETWEEN` is inclusive on the upper bound, which is different than the original
    // version of this.
    let stmt = Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
        SELECT
            status,
            count(1),
            to_timestamp(
                floor(EXTRACT(epoch FROM ended_at) / EXTRACT(epoch FROM interval '5 min'))
                * EXTRACT(epoch FROM interval '5 min')
            ) AS bucket
        FROM messageattempt
        WHERE
            status in (0, 2)
            AND endp_id IN (SELECT id FROM endpoint WHERE app_id = $1)
            AND ended_at >= $2
            AND ended_at < $3
        GROUP BY status, bucket
        ORDER BY bucket ASC;
     "#,
        [app_id.into(), start_date.into(), end_date.into()],
    );
    tracing::trace!("Running query: {}", stmt.to_string());
    let rows: Vec<AppAttemptStatsResult> =
        ctx!(AppAttemptStatsResult::find_by_statement(stmt).all(db).await)?;

    let mut success_rows = BTreeMap::new();
    let mut failure_rows = BTreeMap::new();
    for row in &rows {
        if row.status == 0 {
            success_rows.insert(row.bucket, row.count);
        } else {
            failure_rows.insert(row.bucket, row.count);
        }
    }

    let bucket_count = ((end_date - start_date).num_minutes() / 5)
        .try_into()
        .map_err(|e| err_generic!("{}", e))?;
    let mut success_count = Vec::with_capacity(bucket_count);
    let mut failure_count = Vec::with_capacity(bucket_count);

    let mut i = start_date;
    while i < end_date {
        success_count.push(success_rows.get(&i).copied().unwrap_or_default());
        failure_count.push(failure_rows.get(&i).copied().unwrap_or_default());
        i += chrono::Duration::minutes(5);
    }

    let data = AttemptStatisticsData {
        success_count: if success_count.is_empty() {
            None
        } else {
            Some(success_count)
        },
        failure_count: if failure_count.is_empty() {
            None
        } else {
            Some(failure_count)
        },
    };

    Ok(Json(AttemptStatisticsResponse {
        common_: BaseStatisticsResponse {
            period: StatisticsPeriod::FiveMinutes,
            start_date,
            end_date,
        },
        data,
    }))
}

pub fn router(hide_secret_routes: bool) -> ApiRouter<AppState> {
    let tag = openapi_tag("Stats");
    ApiRouter::new().api_route_with(
        "/stats/app/:app_id/attempt/",
        get_with(get_app_attempt_stats, get_app_attempt_stats_operation),
        |op| op.hidden(hide_secret_routes).with(&tag),
    )
}

/// Rounds the datetime down to the nearest interval of the given `StatisticsPeriod`.
fn round_down_dt(input: DateTime<Utc>, period: StatisticsPeriod) -> Option<DateTime<Utc>> {
    Some(match period {
        StatisticsPeriod::OneDay => input
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?,
        StatisticsPeriod::FiveMinutes => {
            let delta = input.minute() % 5;
            if delta == 0 {
                input.with_second(0)?.with_nanosecond(0)?
            } else {
                input
                    .with_minute(input.minute() - delta)?
                    .with_second(0)?
                    .with_nanosecond(0)?
            }
        }
    })
}

fn validate_date_range(
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    period: StatisticsPeriod,
    default_delta: chrono::Duration,
    max_delta: chrono::Duration,
) -> Result<(DateTime<Utc>, DateTime<Utc>), crate::error::Error> {
    let now = Utc::now();

    let (sd, mut ed) = match (start, end) {
        (None, None) => {
            let ed = now;
            let sd = ed - default_delta;
            (sd, ed)
        }
        (Some(_), None) => {
            return Err(err_validation!("Missing end_date"));
        }
        (None, Some(_)) => {
            return Err(err_validation!("Missing start_date"));
        }
        (Some(sd), Some(ed)) => (sd, ed),
    };

    let invalid_date_range = || err_validation!("Invalid date range");
    if sd >= ed {
        return Err(invalid_date_range());
    } else if ed > sd + max_delta {
        return Err(err_validation!("Date range is too large"));
    }
    // Truncate the end of the range to "now", but what if start is in the future?
    // Future ranges probably don't matter because they won't yield any data.
    if sd < now && now < ed {
        ed = now;
    }

    let sd = round_down_dt(sd, period).ok_or_else(invalid_date_range)?;
    let ed = round_down_dt(ed, period).ok_or_else(invalid_date_range)?;
    tracing::debug!("Range is {sd} to {ed}");
    Ok((sd, ed))
}

#[cfg(test)]
mod tests {
    use super::round_down_dt;
    use crate::v1::endpoints::stats::StatisticsPeriod;
    use chrono::prelude::*;

    #[test]
    fn test_round_down_to_period_oneday_identity() {
        // When input is at midnight, the rounded version should be the same
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let output = round_down_dt(input, StatisticsPeriod::OneDay).unwrap();
        assert_eq!(input, output);
    }
    #[test]
    fn test_round_down_to_period_fivemins_identity() {
        // When input is on a 5 min interval, the rounded version should be the same
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 20, 30, 0).unwrap();
        let output = round_down_dt(input, StatisticsPeriod::FiveMinutes).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_round_down_to_period_oneday_low() {
        // Ensure we round down.
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 23, 55, 55).unwrap();
        let want = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let got = round_down_dt(input, StatisticsPeriod::OneDay).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_round_down_to_period_fivemins_low() {
        // Ensure we round down.
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 20, 5, 10).unwrap();
        let want = Utc.with_ymd_and_hms(2023, 6, 15, 20, 5, 0).unwrap();
        let got = round_down_dt(input, StatisticsPeriod::FiveMinutes).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_round_down_to_period_oneday_high() {
        // When input is at closer to the following day, the rounded version should be rounded _down_ to midnight.
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 23, 55, 55).unwrap();
        let want = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let got = round_down_dt(input, StatisticsPeriod::OneDay).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_round_down_to_period_fivemins_high() {
        // When input is at closer to the _next 5 min interval_, the rounded version should be rounded _down_ to previous.
        let input = Utc.with_ymd_and_hms(2023, 6, 15, 20, 34, 55).unwrap();
        let want = Utc.with_ymd_and_hms(2023, 6, 15, 20, 30, 0).unwrap();
        let got = round_down_dt(input, StatisticsPeriod::FiveMinutes).unwrap();
        assert_eq!(want, got);
    }
}
