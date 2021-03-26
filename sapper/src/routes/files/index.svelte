<Plus on:message={createDir} on:message />
<div
  class="flex flex-col flex-wrap gap-4 py-4"
  use:clickOutside
  on:clickoutside={selected.reset}
>
  <OptionBar />
  <div class="text-2xl text-pink-900">Folders</div>
  {#if folders.length == 0}
    <div class="text-center text-4xl">No Folders</div>
  {/if}
  <div
    class="grid grid-flow-row gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
  >
    {#each folders as folder}
      <Folder {folder} on:dblclick={() => gotoOrGet(folder)} />
    {/each}
  </div>
  <div class="text-2xl text-pink-900">Files</div>
  {#if files.length == 0}
    <div class="text-center text-4xl">No Files</div>
  {/if}
  <div
    class="grid grid-flow-row gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
  >
    {#each files as file}
      <File {file} on:dblclick={() => gotoOrGet(file)} />
    {/each}
  </div>
</div>

<script lang="ts" context="module">
  export async function preload() {
    const res = await this.fetch(`http://127.0.0.1:3030/api/ls`);
    if (res.ok) {
      const jsonBody = await res.json();
      const fileDatas = jsonBody as Metadata[];
      let files = [];
      let folders = [];
      fileDatas.forEach((fileData) => {
        if (fileData.is_dir) {
          folders.push(fileData);
        } else {
          files.push(fileData);
        }
      });
      return {
        files,
        folders,
      };
    }
    return {
      status: 307,
      redirect: '/files/',
    };
  }
</script>

<script lang="ts">
  import Plus from '../../components/Plus.svelte';
  import File from '../../components/File.svelte';
  import Folder from '../../components/Folder.svelte';
  import type { Metadata } from '../../components/Metadata';
  import { selected } from '../../components/Metadata';

  import { goto } from '@sapper/app';
  import { clickOutside } from '../../components/clickOutside.js';
  import OptionBar from '../../components/OptionBar.svelte';
  import { onDestroy } from 'svelte';

  export let files: Metadata[];
  export let folders: Metadata[];

  async function gotoOrGet(fileData: Metadata) {
    if (fileData.is_dir) {
      await goto('/files/' + fileData.path);
      selected.reset();
    } else {
      fetch('http://127.0.0.1:3030/api/dl?path=' + fileData.path)
        .then((resp) => resp.blob())
        .then((blob) => {
          const url = window.URL.createObjectURL(blob);
          const a = document.createElement('a');
          a.style.display = 'none';
          a.href = url;
          // the filename you want
          a.download = fileData.name;
          document.body.appendChild(a);
          a.click();
          window.URL.revokeObjectURL(url);
        });
    }
  }

  async function createDir(event) {
    let response = await fetch(
      'http://127.0.0.1:3030/web/mkdir?dirname=' + event.detail.text
    );
    if (response.ok) {
    }
  }

  async function refreshData() {
    const res = await fetch(`http://127.0.0.1:3030/api/ls`);
    if (res.ok) {
      const jsonBody = await res.json();
      const fileDatas = jsonBody as Metadata[];
      let foldersnew = [];
      let filesnew = [];
      fileDatas.forEach((fileData) => {
        if (fileData.is_dir) {
          foldersnew.push(fileData);
        } else {
          filesnew.push(fileData);
        }
      });
      files = filesnew;
      folders = foldersnew;
    }
  }

  $: console.log($selected);

  onDestroy(() => {
    selected.reset();
  });
</script>
