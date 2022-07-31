#!/bin/bash
LOGIN_RESULT=$(curl http://localhost:8000/login -d '{"username": "Finn", "password": "patstinks"}' -H 'Content-Type: application/json')

TOKEN=$(echo $LOGIN_RESULT | jq -r '.token')

USER_REQUEST=$(curl http://localhost:8000/user -H "Authorization: Bearer $TOKEN")

echo $USER_REQUEST