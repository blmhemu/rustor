<div class="flex flex-col flex-wrap gap-4 py-4">
  <div class="flex flex-row flex-wrap gap-2 divide-y divide-gray-250">
    Folders
  </div>
  <div
    class="grid grid-flow-row gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
  >
    {#each folders as folder}
      <File fileData={folder} on:dblclick={() => gotoOrGet(folder)} />
    {/each}
  </div>
  <div>Files</div>
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
    const slug = encodeURIComponent(page.params.slug.join('/'));
    const res = await this.fetch(`http://127.0.0.1:3030/api/ls?path=` + slug);
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
      status: 301,
      redirect: '/files/',
    };
  }
</script>

<script lang="ts">
  import File from '../../components/File.svelte';
  import type { FileData } from '../../components/File.svelte';
  import { goto } from '@sapper/app';

  export let files: FileData[];
  export let folders: FileData[];

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
</script>
