<div
  class:bg-blue-200={$selected.has(file)}
  class="group cursor-pointer flex flex-col rounded-lg select-none p-2 space-y-2 border-4 border-transparent hover:border-blue-400"
  on:dblclick
  on:click={(event) => selected.smartadd(event, file)}
  use:clickOutside
  on:clickoutside
>
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
    class="feather feather-file self-center"
    ><path
      d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"
    /><polyline points="13 2 13 9 20 9" /></svg
  >
  <p class="select-text text-lg text-center truncate text-gray-600">
    {file.name}
  </p>
</div>

<script lang="ts">
  import type { Metadata } from './Metadata';
  import { selected } from './Metadata';
  import { clickOutside } from './clickOutside.js';

  export let file: Metadata;
</script>
