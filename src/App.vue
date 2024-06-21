<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

async function debug(content: string) {
  await invoke("debug", {content})
}

async function connect() {
  await invoke("connect", {})
}

async function is_login(): Promise<boolean> {
  return await invoke("is_login", {});
}

async function initialize() {
  await debug("Vue is initialized")
  await connect();

  if (await is_login()) {
    // todo: load chat page
  } else {
    // todo: load login page
  }
}

onMounted(initialize)
</script>

<template>
Placeholder
</template>

<style>

</style>
