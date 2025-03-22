#!/bin/bash
set -e

if [ -d /root/workspace/clients/tabby-agent ]; then
  echo "🔧 构建 tabby-agent..."
  cd /root/workspace/clients/tabby-agent
  pnpm install
  pnpm build
else
  echo "⚠️ 警告：/root/workspace/clients/tabby-agent 不存在，跳过构建"
fi

if [ -d /root/workspace/ee/tabby-ui ]; then
  echo "🔧 构建 tabby-ui..."
  cd /root/workspace/ee/tabby-ui
  pnpm install
  pnpm dev &
else
  echo "⚠️ 警告：/root/workspace/ee/tabby-ui 不存在，跳过构建"
fi

echo "🌀 启动 Caddy..."
cd /root/workspace
caddy run --watch --config ee/tabby-webserver/development/Caddyfile &

wait
