use lazy_static::lazy_static;
use prometheus::{CounterVec, Histogram, histogram_opts, labels, opts, register_counter, register_counter_vec, register_gauge, register_histogram, register_histogram_vec};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

lazy_static! {
    pub static ref HTTP_COUNTER: Counter = register_counter!(opts!(
    /*Counter tracks the total number of HTTP requests made by the load balancer.*/
        "klein_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();

    pub static ref HTTP_NUM_REQUESTS: Gauge = register_gauge!(opts!(
    /*Measures the number of HTTP requests at a particular point in time.*/
        "klein_num_http_requests",
        "Number of requests in a particular time",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
    /*This histogram records the latencies of HTTP requests in seconds, allowing us to
    measure and analyze the distribution of request durations.*/
        "klein_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();

    pub static ref HTTP_RESPONSE_STATUS: CounterVec = register_counter_vec!(
        /*This counter vector tracks the number of HTTP responses, categorized by handler and status code,
    allowing us to monitor the status codes returned by our load balancer.*/
        "klein_http_response_status_code",
        "Number of requests in a particular time",
        &["handler","status_code"]
    ).unwrap();
}
