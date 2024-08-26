<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const searchResult = ref("");
const sKey = ref("程刚青");
const sPath = ref("/home/user/文档");

async function search() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  searchResult.value = await invoke("search", { searchKey: sKey.value, searchPath: sPath.value });
}
</script>

<template>
  <form class="row" @submit.prevent="search">
    <input id="key-input" v-model="sKey" placeholder="Enter a search key..." />
    <input id="path-input" v-model="sPath" placeholder="Enter a search path..." />
    <button type="submit">Search</button>
  </form>

  <p>{{ searchResult }}</p>
</template>
