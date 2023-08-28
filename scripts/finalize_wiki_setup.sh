#!/bin/bash

echo '{
    "adminEmail": "admin@admin.com",
    "adminPassword": "password",
    "adminPasswordConfirm": "password",
    "siteUrl": "http://localhost",
    "telemetry": true
}' | http post http://localhost/finalize
