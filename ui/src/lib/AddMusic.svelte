<script lang="ts">
    import { Button } from "$lib/ui/ui/button";
    import { Input } from "$lib/ui/ui/input";
    import { Label } from "$lib/ui/ui/label";
    import * as Tabs from "$lib/ui/ui/tabs";
    import * as Dialog from "$lib/ui/ui/dialog";
    import TextArea from "$lib/TextArea.svelte";

    import { enhance } from "$app/forms";
    import type { ActionResult } from "@sveltejs/kit";

    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files?.[0]) {
            if (target.files[0].size > 300 * 1024 * 1024) {
                alert("File size must be less than 300 MB");
                target.value = "";
                return;
            }
        }
    }

    let artistName = "";
    let artistDescription = "";

    function handleSubmit() {
        return async ({ result }: { result: ActionResult }) => {
            if (result.type === "success") {
                artistName = "";
                artistDescription = "";
                if (document) {
                    (
                        document.getElementById(
                            "artistFile",
                        ) as HTMLInputElement
                    ).value = "";
                }
            }
        };
    }
</script>

<Dialog.Root>
    <Dialog.Trigger>
        <Button variant="secondary" class="bg-slate-800 hover:bg-slate-700">
            Add music
        </Button>
    </Dialog.Trigger>

    <Dialog.Content class="sm:max-w-[425px] bg-slate-900 text-slate-50">
        <Dialog.Header>
            <Dialog.Title class="text-slate-50">Add music</Dialog.Title>
            <Dialog.Description class="text-slate-400">
                Upload recordings and set metadata here.
            </Dialog.Description>
        </Dialog.Header>

        <Tabs.Root value="recording" class="w-full">
            <Tabs.List
                class="grid w-full grid-cols-4 bg-slate-800 rounded-t-lg"
            >
                <Tabs.Trigger
                    value="recording"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Recording
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="piece"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Piece
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="release"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Release
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="artist"
                    class="px-4 py-2 text-sm font-medium transition-all data-[state=active]:bg-slate-700 data-[state=active]:text-slate-50 text-slate-400 hover:text-slate-50"
                >
                    Artist
                </Tabs.Trigger>
            </Tabs.List>

            <Tabs.Content value="recording" class="space-y-4">
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="piece" class="text-right text-slate-400"
                            >Piece*</Label
                        >
                        <Input
                            id="piece"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="release" class="text-right text-slate-400"
                            >Release*</Label
                        >
                        <Input
                            id="release"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="artist" class="text-right text-slate-400"
                            >Artist*</Label
                        >
                        <Input
                            id="artist"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="file" class="text-right text-slate-400"
                            >Recording*</Label
                        >
                        <Input
                            type="file"
                            id="file"
                            accept="audio/*"
                            on:change={handleFileChange}
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                            required
                        />
                    </div>
                </div>
                <div class="flex justify-end mt-4">
                    <Button
                        type="submit"
                        class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                        >Add recording</Button
                    >
                </div>
            </Tabs.Content>

            <Tabs.Content value="piece">
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="title" class="text-right text-slate-400"
                            >Title*</Label
                        >
                        <Input
                            id="title"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="movements" class="text-right text-slate-400"
                            >Movements</Label
                        >
                        <Input
                            type="number"
                            id="title"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            min={0}
                            step={1}
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="composer" class="text-right text-slate-400"
                            >Composer*</Label
                        >
                        <Input
                            id="composer"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="writer" class="text-right text-slate-400"
                            >Songwriter</Label
                        >
                        <Input
                            id="writer"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label
                            for="description"
                            class="text-right text-slate-400">Description</Label
                        >
                        <TextArea
                            rows={6}
                            placeholder="Enter a detailed description..."
                            classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                        />
                    </div>
                </div>
                <div class="flex justify-end mt-4">
                    <Button
                        type="submit"
                        class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                        >Add piece</Button
                    >
                </div>
            </Tabs.Content>

            <Tabs.Content value="release">
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="title" class="text-right text-slate-400"
                            >Release Title*</Label
                        >
                        <Input
                            id="title"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="artist" class="text-right text-slate-400"
                            >Artist*</Label
                        >
                        <Input
                            id="artist"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                            required
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label
                            for="description"
                            class="text-right text-slate-400">Description</Label
                        >
                        <TextArea
                            rows={6}
                            placeholder="Enter a detailed description..."
                            classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="file" class="text-right text-slate-400"
                            >Cover Art</Label
                        >
                        <Input
                            type="file"
                            id="file"
                            accept="image/*"
                            on:change={handleFileChange}
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                        />
                    </div>
                </div>
                <div class="flex justify-end mt-4">
                    <Button
                        type="submit"
                        class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                        >Add release</Button
                    >
                </div>
            </Tabs.Content>

            <Tabs.Content value="artist">
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addartist"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="name" class="text-right text-slate-400"
                                >Artist Name*</Label
                            >
                            <Input
                                id="name"
                                name="name"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                bind:value={artistName}
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="description"
                                class="text-right text-slate-400"
                                >Description</Label
                            >
                            <TextArea
                                rows={6}
                                placeholder="Enter a detailed description..."
                                name="description"
                                classname="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                bind:value={artistDescription}
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="file" class="text-right text-slate-400"
                                >Image</Label
                            >
                            <Input
                                type="file"
                                name="file"
                                id="artistFile"
                                accept="image/*"
                                on:change={handleFileChange}
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                            />
                        </div>
                    </div>
                    <div class="flex justify-end mt-4">
                        <Button
                            type="submit"
                            class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                            >Add artist</Button
                        >
                    </div>
                </form></Tabs.Content
            >
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
