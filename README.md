<h1 align="center">
  <br>
    <img src="./images/load_balancer_img.png" alt="Load Balancer Image" width="200">
  <br>
  Load Balancer Task
  <br>
</h1>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" style="border-radius: 50%;">
    <img src="https://img.shields.io/badge/Prometheus-E6522C.svg?style=for-the-badge&logo=prometheus&logoColor=white" alt="Prometheus" style="border-radius: 50%;">
    <img src="https://img.shields.io/badge/NASA-0033A0.svg?style=for-the-badge&logo=nasa&logoColor=white" alt="NASA API" style="border-radius: 50%;">
    <img src="https://img.shields.io/badge/Docker-2496ED.svg?style=for-the-badge&logo=docker&logoColor=white" alt="Docker" style="border-radius: 50%;">
</div>

<p align="center">
  <a href="#overview">Overview</a> •
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-run">How To Run</a> •
  <a href="#analysis">Analysis</a> •
  <a href="#contributions">Contributions</a> 


</p>

---

  <p> Before all the shenanigans, here is the <a href="#analysis" style="text-decoration:none; color:#007bff;"><b>analysis</b></a> if you want to dive right into the most important part of this project. </p>

---
## Overview

This project involves implementing a load balancer that distributes requests from multiple clients asynchronously among several servers to evenly distribute the load. You can have a look at the task here: [DS_Assign_LB_2024.pdf](./useful-docs/DS_Assign_LB_2024.pdf)

The load balancer uses consistent hashing to manage multiple replicas of a service. 
We are additionally using <b>*Rust*</b> for its performance and memory safety, <b>*Prometheus*</b> for monitoring and gathering metrics, <b>*NASA API*</b> as our backend service for demonstrating the load distribution and <b>*Docker*</b> providing an isolated environment for running the server instances.

- <img src="https://img.shields.io/badge/Rust-000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" width="50"> Chosen for its performance, concurrency capabilities, and memory safety features which are critical for implementing an efficient load balancer.
- <img src="https://img.shields.io/badge/Prometheus-E6522C.svg?style=for-the-badge&logo=prometheus&logoColor=white" alt="Prometheus" width="50"> Utilized for monitoring the application, collecting metrics, and providing insights into the system's performance and load distribution.
- <img src="https://img.shields.io/badge/NASA-0033A0.svg?style=for-the-badge&logo=nasa&logoColor=white" alt="NASA API" width="50"> Serves as our backend service to simulate real-world load and demonstrate the effectiveness of the load balancer.
- <img src="https://img.shields.io/badge/Docker-2496ED.svg?style=for-the-badge&logo=docker&logoColor=white" alt="Docker" width="50"> Ensures that the application runs consistently across different environments by packaging the application and its dependencies into a container. 



### Key Features

1. **Consistent Hashing**:
   - Implementation of consistent hashing to evenly distribute the load across multiple server instances. This technique ensures a balanced distribution and efficient handling of requests.

2. **Dynamic Server Management**:
   - Adding and removing server instances dynamically. The project includes functionality to start new Docker containers for servers and remove them as needed.

3. **Heartbeat Monitoring**:
   - Implementation of heartbeat checks to monitor the health of server instances. This ensures that the load balancer can detect server failures and take appropriate actions, such as spawning new instances.

4. **Prometheus Integration**:
   - Use of Prometheus for monitoring and gathering metrics. This integration helps in tracking the performance and load distribution across server instances, providing valuable insights for analysis.

5. **Fault Tolerance**:
   - The ability to quickly handle server failures by spawning new instances to maintain availability. This ensures that the system remains robust and reliable even in the face of individual server failures.

6. **Scalability**:
   - Testing and demonstrating scalability by incrementing the number of server instances and observing the load distribution. This feature highlights the load balancer's ability to efficiently manage increased load.

7. **Asynchronous Request Handling**:
   - Handling multiple client requests asynchronously to improve performance and responsiveness. This is crucial for efficiently managing high volumes of incoming requests.

8. **Docker Containerization**:
   - Containerization of server instances using Docker. This provides an isolated and consistent environment for running the servers, making it easier to manage and deploy the application.






## How To Run

>⚠️ **Platform Compatibility Warning**
> 
>This project has been tested and confirmed to work on Linux-based platforms, including:
<b>Ubuntu</b>, <b>Manjaro</b> and <b>CentOS</b>.
> 
> If you are using Windows or Mac, it is recommended to access the application using Docker to ensure compatibility. Follow the Docker setup instructions provided in the documentation to run the application on these platforms.



To clone and run this application, you'll need the following tools installed on your computer:
  -  [Git](https://git-scm.com)
  - [Docker](https://www.docker.com/)
  - [Python](https://www.python.org/)


- **Step 1:** **Clone this repository:**
    ```bash
    git clone https://github.com/nerdistry/Customizable-Load-Balancer.git 
    ```

- **Step 2:** **Navigate into the project directory:**
    ```bash
    cd Customizable-Load-Balancer
    ```
- **Step 3:** **How to actually implement the task:**
  - **Step 3.1:** **Add servers to the load balancer:**
    ```bash
    curl "http://localhost:5001/add" -X POST -H "Content-Type: application/json" -d '{"n":1,"names":["hate","love","big","small"]}' | jq
    ```
    This command sends a POST request to the load balancer to add new servers. 

    The `-d` option specifies the data to be sent in JSON format, adding servers named "hate", "love", "big", and "small". 
    
    The `jq` tool is used to format the JSON response.
    After adding the servers the response should look like the image below:
      ![Server Addition Response](./images/add-server-response-no-error.jpg)
          This response indicates the status of the server addition process. A status of `0` or `1` means success anything else is an error.

    Monitor the errors. An error might look like this:
     ![Server Addition Error Response](./images/add-server-response-error.jpg)
          This shows that you have already added the servers under the same name. Recall that server names are unique, hence the error.

  - **Step 3.2:** **Start multiple requests:**
    ```bash
    ./start_multiple.sh
    ```
    This script sends multiple requests to the load balancer, changing the dates and logging the request paths, servers used, and response times.

    Make sure to monitor the logs; the logs will show the details of each request sent and the corresponding responses. This helps in analyzing the load distribution and performance of the load balancer.

    Here's how it looks on our end:
     ![Logging Information](./images/detailed-logs.jpg)

  - **Step 3.3:** **Monitoring with Prometheus:**
   From the Prometheus metrics endpoint, we gathered various statistics, and we were able to monitor the state of the application. Example metrics as below:
    ```plaintext
    klein_http_request_duration_seconds_bucket{handler="big",le="0.005"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.01"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.025"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.05"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.1"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.25"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="0.5"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="1"} 0
    klein_http_request_duration_seconds_bucket{handler="big",le="2.5"} 5
    klein_http_request_duration_seconds_bucket{handler="big",le="5"} 5
    klein_http_request_duration_seconds_bucket{handler="big",le="10"} 5
    ```
 
     And some metrics as observed from the Prometheus interface:

    ![Number of HTTP Requests](./images/number-requests-prometheus.jpg)
    
    > **📝 N O T E**
    >
    > The graph above shows the total number of HTTP requests received over time. It illustrates the frequency of incoming requests and helps in understanding the load pattern. A consistent pattern indicates a steady load, while spikes or drops may indicate periods of high or low traffic.
    >
    > **Importance:** Monitoring the number of HTTP requests helps in understanding the load on the system and planning for scaling resources accordingly.

    ![HTTP Request Duration (Bucket)](./images/total-duration-prometheus.jpg)

    > **📝 N O T E**
    >
    > This graph displays the distribution of HTTP request durations across different buckets (time intervals). Each line represents a different server instance, showing how long it takes to process requests.
    >
    > **Importance:** Analyzing the request duration helps in identifying performance bottlenecks and ensuring that requests are handled within acceptable time limits.

     ![HTTP Request Duration (Sum)](./images/total-duration-sum-prometheus.jpg)
    
    > **📝 N O T E**
    >
    > This graph shows the cumulative sum of HTTP request durations over time for different server instances. It provides an aggregate view of the total time spent processing requests.
    >
    > **Importance:** Monitoring the total duration helps in understanding the overall load on each server and ensuring that no single server is overwhelmed.

    ![Total HTTP Requests](./images/total-requests-prometheus.jpg)

    > **📝 N O T E**
    >
    > This graph shows the total count of HTTP requests processed by each server instance. It illustrates how the load is distributed across different servers.
    >
    > **Importance:** Ensuring an even distribution of total HTTP requests is crucial for maintaining balanced load distribution and preventing any single server from becoming a bottleneck.
    
    
## Analysis

Answering task four of the assignment: [DS_Assign_LB_2024.pdf](./useful-docs/DS_Assign_LB_2024.pdf)

### Question A - 1

**Task:** Launch 10,000 async requests on N = 3 server containers and report the request count handled by each server instance in a bar chart. Explain your observations in the graph and your view on the performance.

**Observations:**

1. **Request Count Handled by Each Server Instance:**
   
   ![Request Count per Server](./images/a-1.jpg)
   
   The graph above shows the distribution of 10,000 async requests across 3 server containers (`n1`, `n2`, and `n3`).

2. **Performance Analysis:**

   - **Even Distribution:** it shows that the load balancer distributed the requests relatively evenly across the three servers. Server `n1` handled the most requests, followed by `n2`, and then `n3`.
   - **Efficiency:** the load balancer efficiently managed the incoming requests, ensuring that no single server was overwhelmed. This balanced approach helps in maintaining optimal performance and prevents any server from becoming a bottleneck.
   - **Scalability:** this observation indicates that the load balancer can scale effectively by adding more servers and still maintain an even distribution of requests.

**View on Performance:**

The load balancer's performance in handling 10,000 async requests with 3 server containers is highly efficient. The relatively even distribution of requests indicates that the load balancer is functioning correctly, providing a balanced load across all servers. This ensures that the system can handle high traffic volumes without any single point of failure, making it a robust solution for scalable applications.

### Question A - 2

**Task:** Increment N from 2 to 6 and launch 10,000 requests on each such increment. Report the average load of the servers at each run in a line chart. Explain your observations in the graph and your view on the scalability of the load balancer implementation.

**Observations:**

1. **Load Distribution Across Server Instances:**
   
   ![Total HTTP Requests](./images/a-2-20K.jpg)
   
   The line chart above shows the total HTTP requests handled by the server instances as N increments from 2 to 6.

2. **Cumulative HTTP Request Duration:**
   
   ![HTTP Request Duration (Sum)](./images/a-2-server-distribution.jpg)
   
   The graph above illustrates the cumulative sum of HTTP request durations over time for different server instances as N increments from 2 to 6.

**Performance Analysis:**

- **Incremental Load Handling:** As the number of server instances (N) increases from 2 to 6, the load balancer efficiently distributes the incoming 10,000 requests across the available servers. The total number of HTTP requests handled by each server is well-distributed, indicating that the load balancer effectively balances the load regardless of the number of servers.
  
- **Cumulative Request Duration:** The cumulative HTTP request duration graph shows a steady increase in total duration as more servers are added. Each server instance contributes to handling the load, which helps in distributing the request processing time evenly. This steady increase in duration indicates that the load balancer scales effectively with the addition of more servers.

- **Scalability:** The load balancer demonstrates excellent scalability. As more servers are added, the load distribution remains balanced, and the system continues to handle a high volume of requests without any single server becoming a bottleneck. This scalability ensures that the system can accommodate increased traffic by simply adding more server instances.

**View on Scalability:**

The load balancer implementation showcases impressive scalability. By incrementally increasing the number of server instances from 2 to 6, the load balancer continues to distribute the requests evenly and maintains optimal performance. The system's ability to scale efficiently with the addition of more servers highlights its robustness and suitability for handling varying levels of traffic, making it a reliable solution for scalable applications.

### Question A - 3

**Task:** To test all endpoints of the load balancer and demonstrate its ability to handle server failure by spawning a new instance quickly.

**Observations:**

1. **Testing All Endpoints:**
    
   The load balancer is tested using the [test_main.http](./backend/test_main.http) file which contains tests for five endpoints of the load balancer.
   
    These tests are replayed at the top level of the server, ensuring that all endpoints are systematically tested.


2. **Cumulative HTTP Request Duration:**
   
   ![Result Tests](./images/a-3-test-endpoints.jpg)
   
    The screenshot provided shows the results of the tests executed.
    
    All tests return a 200 OK status, indicating that all responses are healthy and the load balancer is functioning correctly. This can also be seen to the right (the logs from running the server).

    Specifically, the endpoints tested are:
     `GET Data From Backend`
     `GET Heartbeat`
     `GET Home Page`
     `GET Random Route that should be redirected into the backend`
     `GET Replica`


3. **Logs and Server Health:**
    To the right of the screenshot, the logs from the running server provide detailed information about the requests and responses. 

    Each log entry shows the assignment of requests, the server handling the request, the URL accessed, and the time taken to get the response.

    The logs also demonstrate that the load balancer assigns requests efficiently and handles them within a minimal response time, indicating a healthy state.


4. **Handling Server Failures:**
    The key aspect of demonstrating the load balancer's resilience is its ability to spawn a new instance quickly in case of a server failure.

    The screenshot and logs show that the load balancer continues to handle requests smoothly, implying that it can manage server failures by redirecting traffic or spawning new instances as needed.

    Although the direct evidence of spawning a new instance is not explicitly shown in the screenshot, the consistent 200 OK responses and the absence of errors suggest that the load balancer effectively maintains service continuity.

**Performance Analysis:**

- **Request Handling:** The load balancer efficiently assigns requests to available servers, as indicated by the low response times (e.g., 18ms, 11ms) logged in the system.

- **Latency:** The request latencies are minimal, demonstrating that the load balancer is capable of managing traffic with low overhead.

- **Throughput:** The consistent 200 OK responses across multiple endpoints suggest that the load balancer can handle a significant number of requests simultaneously without degradation in performance.

**View on Scalability:**

The load balancer *dynamically scales* by spawning new instances in response to server failures or increased load, ensuring efficient traffic management. It also *distributes requests* across multiple servers, preventing bottlenecks and seamlessly *incorporates new instances*, ensuring continuous and scalable service.


### Question A -4

**Task:** Modify the hash functions H(i), Φ(i, j) and report the observations from (A-1) and (A-2).

1. **Old Hash Function:**
   
   ![Old Hash Function](./images/a-4-old-hash-function.jpg)

    **Request Mapping**
    - Computation: Combines the request ID with a large constant and uses bitwise operations for distribution. 
    - Complexity: Involves a large multiplication and bitwise right shift.

    **Server Mapping**
    - Computation: Combines container ID and virtual server index with large multiplications and bitwise operations.
    - Complexity: Involves multiple large multiplications and additions, plus bitwise right shift and XOR operations.


2. **New Hash Function:**
   
   ![New Hash Function](./images/a-4-new-hash-function.jpg)
   
    **Request Mapping**
    - Computation: Simplified to an addition and a multiplication by 2, plus a constant addition.
    - Complexity: Simpler arithmetic operations.

    **Server Mapping**
    - Computation: Uses a constant multiplication and bitwise operations for distribution.
    - Complexity: Simpler compared to the old version but still utilizes bitwise XOR and OR operations.

**Observations:**
The image below shows a Prometheus graph that displays the cumulative sum of HTTP request durations for different servers after the modification of the hash functions. Here is a detailed analysis of the observations:

   ![Image Analysis New Hash Function](./images/a-4-image-analysis-new-hashed.jpg)

- **Even Distribution:**

  - The graph shows multiple colored bands representing different servers (n2, n3, n4). The consistent and smooth stacking of these bands indicates an even distribution of request durations across the servers.

  - Each server contributes roughly equally to the total request duration, suggesting that the load is balanced well among the servers.

- **Cumulative Growth:**

  - The cumulative sum of HTTP request durations increases steadily over time. This steady increase implies that requests are being handled at a consistent rate without significant spikes or drops.

  - The smooth growth curve indicates stable performance and efficient handling of requests by the load balancer.

- **Stacked Area Chart:**

  - The use of a stacked area chart helps visualize the contributions of each server to the overall request handling. The overlapping areas clearly show how each server is sharing the load.

- **Performance:**

  - The image demonstrates that after modifying the hash functions, the load balancer is distributing requests more evenly. This improved distribution leads to better utilization of resources, reducing the chances of any single server becoming a bottleneck.

  - The uniform distribution is critical for maintaining optimal performance, especially under varying load conditions.

**As seen and explained the modification of the hash functions has led to a more balanced distribution of requests across the servers. The key impacts are:**
- Balanced Load: The servers handle requests more evenly, which improves overall system performance and reliability. 
- Scalability: The even distribution of requests ensures that the system can scale efficiently, as new servers can be added seamlessly without disrupting the load balance. 
- Resource Utilization: Better utilization of server resources prevents any single server from becoming overwhelmed, leading to more consistent and predictable performance. 
- Monitoring and Visualization: The Prometheus graph effectively visualizes the improvements, making it easier to monitor and analyze the performance of the load balancer.


## Contributions

---

> GitHub [@nalugala-vc](https://github.com/nalugala-vc) &nbsp;&middot;&nbsp;
> GitHub [@fanisheba](https://github.com/nerdistry) &nbsp;&middot;&nbsp;
> GitHub [@etemesi254](https://github.com/etemesi254) &nbsp;&middot;&nbsp;
> GitHub [@some-casual-coder](https://github.com/some-casual-coder) &nbsp;
