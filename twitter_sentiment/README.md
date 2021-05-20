

## Please refer to this link for the demo
https://drive.google.com/file/d/1Uuz7_qmS2gV1zSveobis5EPtcN8jBY91/view?usp=sharing

**[Python][Scala]** This project is a cloud based implementation using Apache Spark, Hadoop, Kafka deployed on a Function-as-a-service platform on AWS.
The project aims to train itself on a database of {tweet, sentiment} records, eventually to predict a tweet and its attached sentiment.
The model is also trained and updated on the fly, by leveraging dictionary and connotations data. Atypical English slangs are referred to Urban Dictionary for its definition.

The attached video is an offline version of the implementation that trains and predicts on COVID-19 related tweets with roughly 85% accuracy.
No implementation code is provided as they are property of the school (HKU), and most of them reference standard API of relevant platforms such as Twitter, Apache Spark, Kafka.
We instead performed tuning on the latency, throughput, and platform resilience, which is shown in the report.
