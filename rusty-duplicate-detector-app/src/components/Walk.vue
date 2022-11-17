<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const path = ref("");
const walkResponse = ref();

async function walk() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("walk", { path: path.value })
    .then((result) => walkResponse.value = result)
    .catch((error) => console.error(error));
}
</script>

<template>
    <div>
      <div class="card">
      <input id="walk-input" v-model="path" placeholder="Enter a path..." />
      <button type="button" @click="walk()">Walk</button>
    </div>
    <p>{{ walkResponse }}</p>
  </div>

</template>
