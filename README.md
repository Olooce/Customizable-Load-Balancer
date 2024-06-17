<h1 align="center">
  <br>
   <a href="http://localhost"><img src="./images/load_balancer_img.webp" alt="Load Balancer Image" width="200" style="border-radius: 50%;"></a>
  <br>
  Load Balancer Task
  <br>
</h1>

<h4 align="center"> Distribution of requests from multiple clients</a>.</h4>

<p align="center">
<a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
</a>
<a href="https://prometheus.io/">
    <img src="https://img.shields.io/badge/Prometheus-E6522C.svg?style=for-the-badge&logo=prometheus&logoColor=white" alt="Prometheus">
</a>
<a href="https://www.nasa.gov/">
    <img src="https://img.shields.io/badge/NASA-0033A0.svg?style=for-the-badge&logo=nasa&logoColor=white" alt="NASA API">
</a>
<a href="https://www.docker.com/">
    <img src="https://img.shields.io/badge/Docker-2496ED.svg?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
</a>



</p>

<p align="center">
  <a href="#overview">Overview</a> •
  <a href="#key-features">Key Features</a> •
  <a href="#download">Download</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

<!-- ![screenshot](https://raw.githubusercontent.com/amitmerchant1990/electron-markdownify/master/app/img/markdownify.gif) -->

## Overview

This project involves implementing a load balancer that distributes requests from multiple clients asynchronously among several servers to evenly distribute the load. 
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

    - **Step 3.2:** **Monitoring with Prometheus:**
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

        ![Number of HTTP requests](./images/number-requests-prometheus.jpg)
        >📝**Note** 
            > 
            > The graph above shows the total number of HTTP requests received over time. It illustrates the frequency of incoming requests and helps in understanding the load pattern. A consistent pattern indicates a steady load, while spikes or drops may indicate periods of high or low traffic. 
            > 
            > Importance: Monitoring the number of HTTP requests helps in understanding the load on the system and planning for scaling resources accordingly.

      ![HTTP Request Duration (Bucket)](./images/total-duration-prometheus.jpg)
        >📝**Note** 
            > 
            > This graph displays the distribution of HTTP request durations across different buckets (time intervals). Each line represents a different server instance, showing how long it takes to process requests. 
            > 
            > Importance: Analyzing the request duration helps in identifying performance bottlenecks and ensuring that requests are handled within acceptable time limits.
      
      ![HTTP Request Duration (Sum)](./images/total-duration-sum-prometheus.jpg)
        >📝**Note** 
            > 
            > This graph shows the cumulative sum of HTTP request durations over time for different server instances. It provides an aggregate view of the total time spent processing requests.
            > 
            > Importance: Monitoring the total duration helps in understanding the overall load on each server and ensuring that no single server is overwhelmed.
    
      ![Total HTTP Requests](./images/total-requests-prometheus.jpg)
        >📝**Note** 
            > 
            > This graph shows the total count of HTTP requests processed by each server instance. It illustrates how the load is distributed across different servers.
            > 
            > Importance: Ensuring an even distribution of total HTTP requests is crucial for maintaining balanced load distribution and preventing any single server from becoming a bottleneck.
      

![Server Addition Response](./images/total-duration-prometheus.jpg)










  




## Credits

This web app uses the following important packages for the:

**(a) Normal Backend**
- [Cloudinary](https://cloudinary.com/)
- [Ethers](https://www.npmjs.com/package/ethers)
- [Express](https://www.npmjs.com/package/express)
- [Formik](https://formik.org/docs/overview)
- [Mongoose](https://www.npmjs.com/package/mongoose)
- [Slugify](https://www.npmjs.com/package/slugify)

There were just worth mentioning, you can check out the rest in the package-lock.json file in the backend folder.

**(a) Solidity Backend**
- [Hardhat](https://hardhat.org/hardhat-runner/docs/getting-started)
- [Ethereum-Waffle](https://ethereum-waffle.readthedocs.io/en/latest/)
- [Chai](https://ethereum-waffle.readthedocs.io/en/latest/)


**(b) Frontend**
- [Redux Toolkit](https://redux-toolkit.js.org/)
- [Testing Library](https://www.npmjs.com/package/@testing-library/react)
- [Template](https://adminlte.io/)
- [Bootstrap](https://getbootstrap.com/)
- [Firebase](https://firebase.google.com/)
- [React](https://reactnative.dev/)

The packages are more than we can mention!

#### Output Structure
---

```shell
Escrow_Web_App/
├── backend/
│   ├── config/
│   │   └── dbConnect.js
│   │   └── jwtToken.js
│   │   └── refreshtoken.js
│   ├── contracts/
│   │   └── escrow.sol
│   ├── controller/
│   │   └── brandCtrl.js
│   │   └── categoryCtrl.js
│   │   └── emailCtrl.js
│   │   └── productCtrl.js
│   │   └── userCtrl.js
│   ├── middlewares/
│   │   └── authMiddleware.js
│   │   └── errprHandler.js
│   │   └── uploadImages.js
│   ├── models/
│   │   └── brandModel.js
│   │   └── cartModel.js
│   │   └── categoryModel.js
│   │   └── orderModel.js
│   │   └── productModel.js
│   │   └── userModel.js
│   ├── routes/
│   │   └── authRoute.js
│   │   └── brandRoute.js
│   │   └── categoryRoute.js
│   │   └── productRoute.js
│   ├── scripts/
│   │   └── deploy.js
│   ├── index.js
│   └── package.json
├── frontend/
│   ├── public/
│   │   └── index.html
│   ├── src/
│   │   ├── components/
│   │   ├── App.js
│   │   └── index.js
│   ├── package.json
│   └── ...
└── README.md

```

## License

MIT

---

> GitHub [@bryanlwaya](https://github.com/BryanLwaya) &nbsp;&middot;&nbsp;
> GitHub [@fanisheba](https://github.com/nerdistry) &nbsp;&middot;&nbsp;