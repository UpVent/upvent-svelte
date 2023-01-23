#!/bin/sh

# Check for command line arguments
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Deployment Script for Sveltekit sites on NGINX

Usage: deploy.sh [options]
Options: -h | --help        Print this screen.

Any other arguments other than -h or --help will result in normal execution.

================================IMPORTANT=======================================
This script needs environment variables to work properly. Please make sure you
set them correctly beforehand. For 'security' reasons we won't read these
variables from a .env file.

However a .env file is still required on the root of this project to properly
copy the production-ready files.

You'll need the following environment variables in order for this script to work
you can leave some 'as is' but this is heavily discouraged:

(This script will assume your default nginx user/group is www-data, change this
to your server specs. It's heavily recommended to use separate users/groups if
you have multiple deployments)

==================================VARIABLES=====================================

- DEPLOY_USER: The SSH/SCP user of the server where you are going to deploy. (\
Default: root)
- DEPLOY_IP: A valid IP address from the server where you are going to deploy.
- DEPLOY_PORT: The SSH/SCP port your server uses. (Default: 22)
- DEPLOY_DIR: The default web-directory your NGINX installation uses. (Default:\
 /var/www/html/example.com/) - Notice the trailing slash!
- DEPLOY_UNIT: The systemd unit you are using to manage your Sveltekit deploy. (\
Default: sveltekit.service)
- DOMAIN_URL: The domain url where your app lives.\
(Default: https://upvent.codes/)
"
    exit 0
fi

# Read needed environment variables for deployment

# The server user where you are going to deploy
[ -z "${DEPLOY_USER}" ] && SERVER_USER="root" || SERVER_USER="${DEPLOY_USER}"

# The server IP where you are going to deploy
[ -z "${DEPLOY_IP}" ] && echo "Variable DEPLOY_IP not found!" ||\
    SERVER_IP="${DEPLOY_IP}"

# The server port to ssh/scp your files into
[ -z "${DEPLOY_PORT}" ] && SERVER_PORT="22" || SERVER_PORT="${DEPLOY_PORT}"

# The deploy directory for this project (check your NGINX configuration for this)
[ -z "${DEPLOY_DIR}" ] && SERVER_DIR="/var/www/html/example.com/"\
    || SERVER_DIR="${DEPLOY_DIR}"

# The Sveltekit systemd service to manage your current deployment
[ -z "${DEPLOY_UNIT}" ] && SERVER_UNIT="sveltekit.service"\
    || SERVER_UNIT="${DEPLOY_UNIT}"

# The domain URL to test
[ -z "${DOMAIN_URL}" ] && DOMAIN_TEST="https://upvent.codes"\
    || DOMAIN_TEST="${DOMAIN_URL}"

# === Main Logic === #

# Check if .env file exists to package it on the distributable .zip
if [ ! -f ".env" ]; then
    echo ".env not found! Follow README.md instructions or call this command\
with -h or --help to setup your project correctly."
    exit 1
fi

# Clean and build the project
npm install && npm run build && npm ci --prod

# Copy the production files to your server
scp -P "$SERVER_PORT" -r build package.json node_modules .env\
"$SERVER_USER"@"$SERVER_IP":"$SERVER_DIR"

# Give permissions to the default NGINX user/group (change this)
ssh -p "$SERVER_PORT" "$SERVER_USER"@"$SERVER_IP" \
"chown -R www-data:www-data $SERVER_DIR"

# Secure .env permissions
ssh -p "$SERVER_PORT" "$SERVER_USER"@"$SERVER_IP" "chmod 644 $SERVER_DIR.env"

# Restart the systemd service
ssh -p "$SERVER_PORT"\
    "$SERVER_USER"@"$SERVER_IP" "systemctl restart $SERVER_UNIT"

# Check the status page

STATUS=$(curl -o /dev/null -s -w "%{http_code}\n" "$DOMAIN_TEST")

if [ "$STATUS" = "200" ]; then
    echo "Deployment appears successful, got response $STATUS. Please double
check if your site is working properly."
    exit 0
elif [ "$STATUS" = "500" ] || [ "$STATUS" = "404" ]; then
    echo "Something went wrong with this deployment. Check your Sveltekit
or NGINX logs to see what went wrong."
    exit 1
fi

# Assume everything got executed and exit with a 0 status
exit 0
