export interface PostOptions {
    idempotencyKey?: string;
}

export interface ListOptions {
    iterator?: string | null;
    limit?: number;
}
