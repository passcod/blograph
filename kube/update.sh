#!/usr/bin/bash

version=$(jq -r '.version' package.json)

kubectl patch deployment blograph -p <<PATCH
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
