<script lang="ts">
    import type { Recording } from "$lib/types";
    import * as Table from "$lib/ui/ui/table";
    import { Button } from "$lib/ui/ui/button";
    import { Play, MoreHorizontal } from "lucide-svelte";
    import * as DropdownMenu from "$lib/ui/ui/dropdown-menu";

    export let recordings: Recording[];

    function formatDuration(seconds: number): string {
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
    }

    function handlePlay(recording: Recording) {
        console.log("Playing:", recording.pieceName);
    }
</script>

<div class="relative w-full overflow-auto">
    <Table.Root class="w-full">
        <Table.Header class="bg-slate-900/50">
            <Table.Row>
                <Table.Head class="w-[40px]">#</Table.Head>
                <Table.Head>Title</Table.Head>
                <Table.Head class="hidden md:table-cell">Composers</Table.Head>
                <Table.Head class="hidden md:table-cell">Performers</Table.Head>
                <Table.Head class="w-[100px]"></Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each recordings as recording}
                <Table.Row class="group hover:bg-slate-800/50">
                    <Table.Cell class="font-medium text-slate-400">
                        <div class="flex items-center">
                            <span class="group-hover:hidden"
                                >{recording.trackNumber}</span
                            >
                            <Button
                                variant="ghost"
                                size="icon"
                                class="hidden group-hover:inline-flex h-8 w-8"
                                on:click={() => handlePlay(recording)}
                            >
                                <Play class="h-4 w-4" />
                            </Button>
                        </div>
                    </Table.Cell>
                    <Table.Cell>
                        <div class="flex flex-col">
                            <span class="font-medium text-slate-200"
                                >{recording.pieceName}</span
                            >
                            <span class="text-sm text-slate-400 md:hidden">
                                {recording.piece.composers
                                    .map((c) => c.name)
                                    .join(", ")}
                            </span>
                        </div>
                    </Table.Cell>
                    <Table.Cell class="hidden md:table-cell text-slate-400">
                        {recording.piece.composers
                            .map((c) => c.name)
                            .join(", ")}
                    </Table.Cell>
                    <Table.Cell class="hidden md:table-cell text-slate-400">
                        {recording.performers.map((p) => p.name).join(", ")}
                    </Table.Cell>
                    <Table.Cell>
                        <div class="flex justify-end">
                            <DropdownMenu.Root>
                                <DropdownMenu.Trigger asChild let:builder>
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        builders={[builder]}
                                        class="h-8 w-8 opacity-0 group-hover:opacity-100"
                                    >
                                        <MoreHorizontal class="h-4 w-4" />
                                    </Button>
                                </DropdownMenu.Trigger>
                                <DropdownMenu.Content
                                    class="w-48 bg-slate-900 border-slate-800"
                                >
                                    <DropdownMenu.Item
                                        class="text-slate-200 hover:bg-slate-800"
                                    >
                                        Add to Queue
                                    </DropdownMenu.Item>
                                    <DropdownMenu.Item
                                        class="text-slate-200 hover:bg-slate-800"
                                    >
                                        Add to Playlist
                                    </DropdownMenu.Item>
                                    <DropdownMenu.Separator
                                        class="bg-slate-800"
                                    />
                                    <DropdownMenu.Item
                                        class="text-slate-200 hover:bg-slate-800"
                                    >
                                        Go to Piece
                                    </DropdownMenu.Item>
                                    <DropdownMenu.Item
                                        class="text-slate-200 hover:bg-slate-800"
                                    >
                                        Share
                                    </DropdownMenu.Item>
                                </DropdownMenu.Content>
                            </DropdownMenu.Root>
                        </div>
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
