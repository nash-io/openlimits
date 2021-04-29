use nash_native_client::Client;
use crate::shared::Result;
use crate::model::Paginator;
use crate::errors::OpenLimitsError;
use crate::shared::timestamp_to_utc_datetime;
use std::convert::TryFrom;
use super::NashParameters;

pub async fn client_from_params_failable(params: NashParameters) -> Result<Client> {
    let client = match params.credentials {
        Some(credentials) => {
            Client::from_keys(
                &credentials.secret,
                &credentials.session,
                params.affiliate_code,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
        None => {
            Client::from_keys_path(
                None,
                None,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
    };

    if let Some(interval) = params.sign_states_loop_interval {
        client.start_background_sign_states_loop(interval);
    }

    Ok(client)
}

pub fn try_split_paginator(
    paginator: Option<Paginator>,
) -> crate::shared::Result<(
    Option<String>,
    Option<i64>,
    Option<nash_protocol::types::DateTimeRange>,
)> {
    Ok(match paginator {
        Some(paginator) => (
            paginator.before,
            match paginator.limit {
                Some(v) => Some(i64::try_from(v).map_err(|_| {
                    OpenLimitsError::InvalidParameter(
                        "Couldn't convert paginator limit to i64".to_string(),
                    )
                })?),
                None => None,
            },
            if paginator.start_time.is_some() && paginator.end_time.is_some() {
                Some(nash_protocol::types::DateTimeRange {
                    start: paginator.start_time.map(timestamp_to_utc_datetime).unwrap(),
                    stop: paginator.end_time.map(timestamp_to_utc_datetime).unwrap(),
                })
            } else {
                None
            },
        ),
        None => (None, None, None),
    })
}