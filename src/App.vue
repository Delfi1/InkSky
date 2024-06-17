<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

async function debug(content: string) {
  await invoke("debug", {content})
}

async function initialize() {
  await debug("Vue is initialized")

}

onMounted(initialize)
</script>

<template>
<h2>Test</h2>
</template>

<style>

</style>
