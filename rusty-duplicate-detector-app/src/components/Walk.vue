<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from '@tauri-apps/api/event'
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';


const path = ref("");
const walkResponse = ref();
console.warn(">>> Setup")

const unlisten = appWindow.listen('path-received', (event) => {
  console.warn(">> ", event);
  let payload = event.payload as { path: string };
  walkResponse.value = walkResponse.value ? `${walkResponse.value} ${payload.path}` : payload.path;
})


function walk() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke("walk", { path: path.value })
      .then((result) => console.info("Walk done"))
      .catch((error) => console.error(error));
    const webview = new WebviewWindow('window')
    webview.emit('walk-started', path.value)
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
