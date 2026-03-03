# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    RateLimiterCheckIn,
    RateLimiterCheckOut,
    RateLimiterGetRemainingIn,
    RateLimiterGetRemainingOut,
)


class RateLimiterAsync(ApiBase):
    async def limit(
        self,
        rate_limiter_check_in: RateLimiterCheckIn,
    ) -> RateLimiterCheckOut:
        """Rate Limiter Check and Consume"""
        body = rate_limiter_check_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limiter/limit",
            body=body,
            response_type=RateLimiterCheckOut,
        )

    async def get_remaining(
        self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
    ) -> RateLimiterGetRemainingOut:
        """Rate Limiter Get Remaining"""
        body = rate_limiter_get_remaining_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limiter/get-remaining",
            body=body,
            response_type=RateLimiterGetRemainingOut,
        )


class RateLimiter(ApiBase):
    def limit(
        self,
        rate_limiter_check_in: RateLimiterCheckIn,
    ) -> RateLimiterCheckOut:
        """Rate Limiter Check and Consume"""
        body = rate_limiter_check_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limiter/limit",
            body=body,
            response_type=RateLimiterCheckOut,
        )

    def get_remaining(
        self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
    ) -> RateLimiterGetRemainingOut:
        """Rate Limiter Get Remaining"""
        body = rate_limiter_get_remaining_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limiter/get-remaining",
            body=body,
            response_type=RateLimiterGetRemainingOut,
        )
