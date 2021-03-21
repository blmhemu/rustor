<div
  class="grid grid-flow-row-dense gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
>
  {#each files as file}
    <File fileData={file} on:dblclick={() => gotoOrGet(file)} />
  {/each}
</div>

<script lang="ts" context="module">
  export async function preload(page, session) {
    const slug = encodeURIComponent(page.params.slug.join('/'));
    const res = await this.fetch(`http://127.0.0.1:3030/api/ls?path=` + slug);
    if (res.ok) {
      const article = await res.json();
      const files = article as FileData[];
      return {
        files,
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

  async function gotoOrGet(fileData: FileData) {
    if (fileData.is_dir) {
      console.log('transfering to ' + '/files/' + fileData.path);
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
