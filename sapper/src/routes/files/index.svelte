<Plus />
<div class="flex flex-col flex-wrap gap-4 py-4">
  <div class="text-2xl text-pink-900">Folders</div>
  {#if folders.length == 0}
    <div class="text-center text-4xl">No Folders</div>
  {/if}
  <div
    class="grid grid-flow-row gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
  >
    {#each folders as folder}
      <File
        fileData={folder}
        bind:selected={selectlist[folder.path]}
        on:dblclick={() => gotoOrGet(folder)}
        on:click={(e) => addToSelected(folder.path, e)}
      />
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
      <File fileData={file} on:dblclick={() => gotoOrGet(file)} />
    {/each}
  </div>
</div>

<script lang="ts" context="module">
  export async function preload(page, session) {
    const res = await this.fetch(`http://127.0.0.1:3030/api/ls`);
    if (res.ok) {
      const jsonBody = await res.json();
      const fileDatas = jsonBody as FileData[];
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
  import File from '../../components/File.svelte';
  import type { FileData } from '../../components/File.svelte';
  import { goto } from '@sapper/app';
  import Plus from '../../components/Plus.svelte';

  export let files: FileData[];
  export let folders: FileData[];

  let selectlist = {};
  for (var file of folders) {
    selectlist[file.path] = false;
  }

  async function gotoOrGet(fileData: FileData) {
    if (fileData.is_dir) {
      await goto('/files/' + fileData.path);
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

  async function addToSelected(params: string, event) {
    if (event.metaKey) {
      selectlist[params] = !selectlist[params];
      console.log('Selected ' + JSON.stringify(selectlist));
    } else {
      for (var file of folders) {
        selectlist[file.path] = false;
      }
      selectlist[params] = true;
    }
  }
</script>
