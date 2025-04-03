#!/bin/bash
if [[ -z $APP_ID ]]; then
  export GH_CHK_IN_APP_ID=$2
else
  export GH_CHK_IN_APP_ID=$APP_ID
fi

if [[ -z $INSTALL_ID ]]; then
  export GH_CHK_IN_INSTALLATION_ID=$2
else
  export GH_CHK_IN_INSTALLATION_ID=$INSTALL_ID
fi

if [[ -z $APP_PKEY ]]; then
  pkey=$3
else
  pkey=$APP_PKEY
fi

echo "$pkey" >> /temp.private-key.pem
export GH_CHK_IN_PRIVATE_KEY_FILE=/temp.private-key.pem

name=$DRONE_STAGE_NAME
head_sha=$DRONE_COMMIT_SHA
details_url=https://drone.planchon.xyz/$DRONE_REPO/$DRONE_BUILD_NUMBER/$DRONE_STAGE_NUMBER/1
stage_status="completed" # Can be 'queued', 'in_progress' or 'completed'
repo=$DRONE_REPO
started_at=$(date -d @$DRONE_BUILD_STARTED +'%Y-%m-%dT%H:%M:%SZ')
completed_at=$(date -d @$DRONE_BUILD_FINISHED +'%Y-%m-%dT%H:%M:%SZ')
conclusion=$DRONE_BUILD_STATUS
title="Drone Status: $conclusion"
summary="Commit: [$(echo $DRONE_COMMIT | awk '{print substr($0,1,7)}')]($DRONE_COMMIT_LINK)\n> $DRONE_COMMIT_MESSAGE\n- **Triggered on:** $DRONE_BUILD_EVENT\n- **Author:** $DRONE_COMMIT_AUTHOR <$DRONE_COMMIT_AUTHOR_EMAIL>\n- **Failed Steps:** $DRONE_FAILED_STEPS"
output='{"title":"'$title'","summary":"'$summary'","text":"Made with ♡ by @joxcat, using https://github.com/webknjaz/check-in."}'

check-in \
  --repo-slug=$repo \
  --name=$name \
  --details-url=$details_url \
  --status=$stage_status \
  --conclusion=$conclusion \
  --started-at=$started_at \
  --completed-at=$completed_at \
  --user-agent="Joxcat's Drone CICD" \
  --output="$output" \
  post-check --head-sha="$head_sha"
