<!-- TODO: See if this can be merged into [...slug].svelte -->
<script lang="ts" context="module">
    export async function load({ fetch, session, page, context }) {
        const res = await fetch(`http://127.0.0.1:3030/api/ls?path=`);
        if (res.ok) {
            const article = await res.json();
            const files = article as FileData[];
            return {
                props: {
                    files,
                },
            };
        }
        return {
            status: res.status,
            error: new Error(`Could not find the resource.`),
        };
    }
</script>

<script lang="ts">
    import { goto } from "$app/navigation";
    import File from "$lib/File.svelte";
    import type { FileData } from "$lib/File.svelte";

    export let files: FileData[];

    async function gotoOrGet(fileData: FileData) {
        if (fileData.is_dir) {
            goto("/files/" + fileData.path);
        } else {
            fetch("http://127.0.0.1:3030/api/dl?path=" + fileData.path)
                .then((resp) => resp.blob())
                .then((blob) => {
                    const url = window.URL.createObjectURL(blob);
                    const a = document.createElement("a");
                    a.style.display = "none";
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

<div
    class="grid grid-flow-row-dense gap-2 grid-cols-3 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-7"
>
    {#each files as file}
        <File fileData={file} on:dblclick={() => gotoOrGet(file)} />
    {/each}
</div>
