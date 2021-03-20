<script lang="ts" context="module">
    export type FileData = { name: string; is_dir: boolean; path: boolean };
</script>

<script lang="ts">
    export let fileData: FileData;
    let icon = fileData.is_dir
        ? "../../../static/folder.svg"
        : "../../../static/file.svg";
</script>

<div
    class="cursor-pointer rounded-lg mx-auto p-3 hover:bg-blue-200"
    on:dblclick
>
    <img class="block" src={icon} alt="Folder" />
    <p class="text-lg text-center truncate text-black-600">
        {fileData.name}
    </p>
</div>
