<div
  class="group cursor-pointer flex flex-col rounded-lg select-none p-2 space-y-2 border-4 border-transparent hover:border-blue-400"
  on:dblclick
>
  <!-- <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="feather feather-download self-end"
    ><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
      points="7 10 12 15 17 10"
    /><line x1="12" y1="15" x2="12" y2="3" /></svg
  > -->
  {#if fileData.is_dir}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="100"
      height="100"
      viewBox="0 0 24 24"
      fill="gray"
      stroke="currentColor"
      stroke-width="1"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="feather feather-folder"
      ><path
        d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
      /></svg
    >
  {:else}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="100"
      height="100"
      viewBox="0 0 24 24"
      fill="gray"
      stroke="currentColor"
      stroke-width="1"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="feather feather-file"
      ><path
        d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"
      /><polyline points="13 2 13 9 20 9" /></svg
    >
  {/if}
  <p class="select-text text-lg text-center truncate text-gray-600">
    {fileData.name}
  </p>
  <button
    class="rounded-lg bg-red-300 transition-opacity delay-100 duration-100 opacity-0 ease-in-out group-hover:opacity-100 ring ring-pink-600"
    >Download</button
  >
</div>

<script lang="ts" context="module">
  export type FileData = { name: string; is_dir: boolean; path: boolean };
</script>

<script lang="ts">
  export let fileData: FileData;
</script>
