<script lang="ts">
    import { Button } from "$lib/ui/ui/button";
    import { Input } from "$lib/ui/ui/input";
    import { Label } from "$lib/ui/ui/label";
    import * as Tabs from "$lib/ui/ui/tabs";
    import * as Dialog from "$lib/ui/ui/dialog";

    let file: File | null = null;

    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files?.[0]) {
            if (target.files[0].size > 300 * 1024 * 1024) {
                alert("File size must be less than 300 MB");
                target.value = "";
                return;
            }
            file = target.files[0];
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

        <Tabs.Root value="recording" class="w-full">
            <Tabs.List
                class="grid w-full grid-cols-2 bg-slate-800 rounded-t-lg"
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
            </Tabs.List>

            <Tabs.Content value="recording" class="space-y-4">
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="piece" class="text-right text-slate-400"
                            >Piece</Label
                        >
                        <Input
                            id="piece"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="artist" class="text-right text-slate-400"
                            >Artist</Label
                        >
                        <Input
                            id="artist"
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50"
                        />
                    </div>

                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="file" class="text-right text-slate-400"
                            >Recording</Label
                        >
                        <Input
                            type="file"
                            id="file"
                            accept="audio/*"
                            on:change={handleFileChange}
                            class="col-span-3 bg-slate-800 border-slate-700 text-slate-50 file:bg-slate-700 file:text-slate-50 file:border-0"
                        />
                    </div>
                </div>
            </Tabs.Content>

            <Tabs.Content value="piece">
                <div class="space-y-4 py-4">
                    <p class="text-sm text-slate-400">
                        Piece metadata management coming soon.
                    </p>
                </div>
            </Tabs.Content>
        </Tabs.Root>

        <Dialog.Footer>
            <Button
                type="submit"
                class="bg-slate-800 hover:bg-slate-700 text-slate-50"
                >Save changes</Button
            >
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
