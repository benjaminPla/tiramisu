#[macro_export]
macro_rules! error {
    ($status:expr) => {{
        use axum::http::StatusCode;
        let message = match $status {
            StatusCode::NOT_FOUND => "Not found",
            StatusCode::UNAUTHORIZED => "Unauthorized",
            _ => "Internal server error",
        };
        eprintln!(
            "{}:{} - HTTP {} {}",
            file!(),
            line!(),
            $status.as_u16(),
            message
        );
        ($status, message.to_string())
    }};

    ($status:expr, $custom_msg:expr) => {{
        eprintln!(
            "{}:{} - HTTP {} {}",
            file!(),
            line!(),
            $status.as_u16(),
            $custom_msg
        );
        ($status, $custom_msg.to_string())
    }};

    ($status:expr, err: $error:expr) => {{
        eprintln!(
            "{}:{} - HTTP {} - {:#?}",
            file!(),
            line!(),
            $status.as_u16(),
            $error
        );
        ($status, "Internal server error".to_string())
    }};
}
