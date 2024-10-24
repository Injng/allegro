<script lang="ts">
    import * as Alert from "$lib/ui/ui/alert";
    import { Button } from "$lib/ui/ui/button";
    import * as Command from "$lib/ui/ui/command";
    import { Input } from "$lib/ui/ui/input";
    import { Label } from "$lib/ui/ui/label";
    import * as Tabs from "$lib/ui/ui/tabs";
    import * as Dialog from "$lib/ui/ui/dialog";
    import TextArea from "$lib/TextArea.svelte";

    import { enhance } from "$app/forms";
    import type { ActionResult } from "@sveltejs/kit";
    import { fade } from "svelte/transition";
    import type { Artist } from "./types";
    import axios from "axios";

    // ensure files are less than 300 MB
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

    // handle submission by displaying alert for success or error
    let showAlert = false;
    let alertType: "success" | "error" = "success";
    let alertMessage = "";
    function handleSubmit() {
        return async ({ result }: { result: ActionResult }) => {
            console.log(result);
            if (result.type === "success") {
                alertType = "success";
                alertMessage = "Successfully added";
                showAlert = true;
            } else if (result.type === "failure") {
                alertType = "error";
                alertMessage = `Error: ${result.data?.error || "An unknown error occurred"}`;
                showAlert = true;
            } else if (result.type === "redirect") {
                alertType = "error";
                alertMessage = "Error: Server API error";
                showAlert = true;
            }

            // hide alert after a few seconds
            setTimeout(() => {
                showAlert = false;
            }, 3000);
        };
    }

    // open search dialog
    let open = false;

    // handle search dialog select
    let selectedArtistName = "";
    let selectedArtistId: number = 0;
    function handleArtistSelect(artist: Artist) {
        selectedArtistName = artist.name;
        selectedArtistId = artist.id;
        open = false;
    }

    // set axios api
    const api = axios.create({
        baseURL: "http://localhost:9000",
    });

    // search for artists
    let artistSearch = "";
    let loadingArtists = true;
    let artists: Artist[] = [];
    async function searchArtists(
        artist_type: "performer" | "composer" | "songwriter",
    ) {
        open = true;
        if (!loadingArtists) {
            return;
        }
        loadingArtists = true;
        try {
            const response = await api.get(`/music/get/${artist_type}s`);
            console.log(response.data);
            for (let a of response.data.message) {
                let artist: Artist = {
                    id: a.id,
                    name: a.name,
                    description: a.description,
                    artist_type: artist_type,
                    image_path: a.image_path,
                };
                artists.push(artist);
            }
        } catch (error) {
            console.error("Error fetching artists:", error);
            artists = [];
        } finally {
            loadingArtists = false;
        }
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

        <div class="fixed bottom-4 left-4 z-100">
            {#if showAlert}
                <div out:fade={{ duration: 300 }}>
                    <Alert.Root
                        variant={alertType === "success"
                            ? "default"
                            : "destructive"}
                        class="mb-2 max-w-md"
                    >
                        <Alert.Title
                            >{alertType === "success"
                                ? "Success"
                                : "Error"}</Alert.Title
                        >
                        <Alert.Description>{alertMessage}</Alert.Description>
                    </Alert.Root>
                </div>
            {/if}
        </div>

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
                <form
                    method="POST"
                    enctype="multipart/form-data"
                    action="?/addrelease"
                    use:enhance={handleSubmit}
                >
                    <div class="grid gap-4 py-4">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="title" class="text-right text-slate-400"
                                >Release Title*</Label
                            >
                            <Input
                                id="title"
                                name="name"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="performer"
                                class="text-right text-slate-400"
                                >Performer*</Label
                            >
                            <div class="col-span-3">
                                <Button
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                                    on:click={() => {
                                        searchArtists("performer");
                                    }}
                                >
                                    {selectedArtistName ||
                                        "Search for an artist..."}
                                </Button>
                                <input
                                    type="hidden"
                                    name="performer"
                                    value={selectedArtistId}
                                />
                                <Command.Dialog
                                    bind:open
                                    class="bg-slate-900 border border-slate-700"
                                >
                                    <Command.Input
                                        placeholder="Search artists..."
                                        class="border-none bg-slate-900 text-slate-50 placeholder:text-slate-400"
                                        bind:value={artistSearch}
                                    />
                                    <Command.List
                                        class="bg-slate-900 text-slate-50"
                                    >
                                        {#if loadingArtists}
                                            <Command.Empty
                                                class="py-6 text-center text-sm text-slate-400"
                                            >
                                                Loading...
                                            </Command.Empty>
                                        {:else}
                                            <Command.Group class="p-1">
                                                {#each artists as artist (artist.id)}
                                                    <Command.Item
                                                        value={artist.name}
                                                        onSelect={() =>
                                                            handleArtistSelect(
                                                                artist,
                                                            )}
                                                        class="cursor-pointer select-none relative text-slate-400 flex items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-slate-700 aria-selected:text-slate-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-slate-800"
                                                    >
                                                        {artist.name}
                                                    </Command.Item>
                                                {/each}
                                            </Command.Group>
                                        {/if}
                                    </Command.List>
                                </Command.Dialog>
                            </div>
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label
                                for="description"
                                class="text-right text-slate-400"
                                >Description</Label
                            >
                            <TextArea
                                rows={6}
                                name="description"
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
                                name="file"
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
                </form></Tabs.Content
            >

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
                                required
                            />
                        </div>

                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="type" class="text-right text-slate-400"
                                >Artist Type*</Label
                            >
                            <select
                                id="type"
                                name="type"
                                class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 rounded-md px-3 py-2"
                                required
                            >
                                <option value="">Select type...</option>
                                <option value="performer">Performer</option>
                                <option value="composer">Composer</option>
                                <option value="songwriter">Songwriter</option>
                            </select>
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
