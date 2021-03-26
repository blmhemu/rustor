<div
  class:bg-blue-200={$selected.has(folder)}
  class="group cursor-pointer flex flex-col rounded-lg select-none p-2 space-y-2 border-4 border-transparent hover:border-blue-400"
  on:dblclick
  on:click={(event) => selected.smartadd(event, folder)}
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
    class="feather feather-folder self-center"
    ><path
      d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
    /></svg
  >
  <p class="select-text text-lg text-center truncate text-gray-600">
    {folder.name}
  </p>
</div>

<script lang="ts">
  import type { Metadata } from './Metadata';
  import { selected } from './Metadata';
  import { clickOutside } from './clickOutside.js';

  export let folder: Metadata;
</script>
