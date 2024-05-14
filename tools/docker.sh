#!/bin/sh

set -e

dname=$(dirname $0)
command=""

if [ -n "$1" ]; then
  command="$1"
  shift
fi

if [ "$command" = "build" ]; then
  args=""

  if [ -n "$1" ]; then
    args="$args -t dorobin/nuts:$1"
    args="$args --build-arg BRANCH=v$1"
  fi

  docker build $args $dname
fi

if [ "$command" = "tag" ]; then
  test -n "$1"

  docker tag dorobin/nuts:$1 dorobin/nuts:latest
  echo "Done"
fi

if [ "$command" = "push" ]; then
  while (( "$#" )); do
    docker push dorobin/nuts:$1
    shift
  done
fi
