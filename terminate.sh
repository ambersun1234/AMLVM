#!/bin/bash

for i in $(ps aux | grep -v "grep" | grep node/app.js | awk '{print $2}'); do
	kill -9 ${i}
done
