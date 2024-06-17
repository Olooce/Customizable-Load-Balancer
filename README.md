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



>⚠️ **Platform Compatibility Warning**
> 
>This project has been tested and confirmed to work on Linux-based platforms, including:
<b>Ubuntu</b>, <b>Manjaro</b> and <b>CentOS</b>.
> 
> If you are using Windows or Mac, it is recommended to access the application using Docker to ensure compatibility. Follow the Docker setup instructions provided in the documentation to run the application on these platforms.




## How To Run

To clone and run this application, you'll need [Git](https://git-scm.com) and [Node.js](https://nodejs.org/en/download/) (which comes with [npm](http://npmjs.com)) installed on your computer. From your command line:

```bash
# Clone this repository
$ git clone https://github.com/nerdistry/EscrowWebApp.git 

# Delete the node modules and package-lock files in both the front and back end folders.

# Go into the front end folder
$ cd frontend

# Install dependencies for the front end.
$ npm install

#Go back a directory
$ cd ..

# Go into the back end folder
$ cd backend

# Install dependencies for the backend end.
$ npm install

#Install hardhat
$ npx hardhat

#Go back a directory
$ cd ..

# Go into the front end folder
$ cd frontend

# Run the app
$ npm start

```

> **Note**
> 
>Remember for Windows you have to have XAMPP installed. And the database we are using is MongoDB.
>
> ⚠️⚠️ We're using testnets for this web application and for security reasons, it is highly recommended you follow suit. 
>
<!-- > If you're stuck breathe in-out then check the above gif. -->


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