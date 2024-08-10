import json
from kafka import KafkaProducer
from datetime import datetime
producer = KafkaProducer(bootstrap_servers=['127.0.0.1:9092'], api_version=(0, 10), value_serializer=lambda m: json.dumps(m).encode('ascii'))
# produce asynchronously
start=0
end=300
while start < end:
    producer.send('hubtel.sms.send',value =
    {
        "Value":start
    },headers=[("Content-Type",b'application/json')]
    )
    start = start + 1
    print("produced")
    producer.flush()