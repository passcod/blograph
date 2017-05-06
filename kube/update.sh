#!/usr/bin/bash

version=$(jq -r '.version' package.json)

patch=$(cat <<PATCH
{
  "spec": {
    "template": {
      "spec": {
        "containers": [
          {
            "name": "blograph",
            "image":"gcr.io/passcod-services/github-passcod-blograph:v$version"
          }
        ]
      }
    }
  }
}
PATCH
)

printf %s "$patch"
echo
echo "> "
kubectl patch deployment blograph -p "$patch"
