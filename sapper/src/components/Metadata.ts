import { writable } from 'svelte/store';
import { onDestroy } from 'svelte';

export type Metadata = { name: string; is_dir: boolean; path: string };

function createSelected() {
    const { subscribe, set, update } = writable(new Set<Metadata>());

    let add = (item: Metadata) => update(f => { f.add(item); return f });
    let reset = () => set(new Set<Metadata>());
    let addOrRemove = (item: Metadata) => update(f => {
        if (f.has(item)) { f.delete(item); }
        else { f.add(item); }
        return f
    });

    return {
        subscribe,
        reset,
        smartadd: (event, item) => {
            if (event.ctrlKey || event.metaKey) {
                addOrRemove(item);
            } else {
                reset();
                add(item);
            }
        }
    }
};

export const selected = createSelected();