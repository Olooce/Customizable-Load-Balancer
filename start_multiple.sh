#!/bin/bash
j=1
for i in {1..100}
do
    # Define start and end dates based on the loop variables
    start="$j day ago";
    end="${i} day ago";
    
    # Increment j by 1
    j=$((j+1));

    # Format the start and end dates
    start_date="date -d '${start}' '+%Y-%m-%d'";
    end_date="date -d '${end}' '+%Y-%m-%d'";

    # Evaluate the date commands to get the actual dates
    ss=$(eval "$start_date");
    ee=$(eval "$end_date");

    # Uncomment the following line to debug and see the dates being processed
    #echo  $ee,$ss,$start_date,$end_date

    # Send a GET request to the local NEO endpoint with the start and end dates
    curl "http://localhost:5001/neo?start_date=$ss&&end_date=$ee" >> /dev/null
    
    # Pause for 5 seconds before the next iteration
    sleep 5
done
