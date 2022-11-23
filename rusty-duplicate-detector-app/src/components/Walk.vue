<script setup lang="ts">
import { ref } from "vue";
import { emit, listen } from '@tauri-apps/api/event'


const path = ref("");
const walkResponse = ref();
console.warn(">>> Setup")

const unlisten = listen('path-found', (event) => {
  console.warn(">> ", event);
  let payload = event.payload as { path: string };
  walkResponse.value = walkResponse.value ? `${walkResponse.value} ${payload.path}` : payload.path;
})


function walk() {
    emit('walk-started', { path: path.value });
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
