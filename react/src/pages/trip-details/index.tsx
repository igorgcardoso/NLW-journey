import { Plus } from "lucide-react";
import { useState } from "react";
import { Activities } from "./activities";
import { CreateActivityModal } from "./create-activity-modal";
import { DestinationAndDateHeader } from "./destination-and-date-header";
import { Guests } from "./guest";
import { ImportantLinks } from "./important-links";

export function TripDetailsPage() {
  const [isCreateActivityModalOpen, setIsCreateActivityModalOpen] =
    useState(false);

  return (
    <div className="mx-auto max-w-6xl space-y-8 px-6 py-10">
      <DestinationAndDateHeader />

      <main className="flex gap-16 px-4">
        <div className="flex-1 space-y-6">
          <div className="flex items-center justify-between">
            <h2 className="text-3xl font-semibold">Atividades</h2>

            <button
              onClick={() => setIsCreateActivityModalOpen(true)}
              className="flex items-center gap-2 rounded-lg bg-lime-300 px-5 py-2 font-medium text-lime-950 hover:bg-lime-400"
            >
              <Plus className="size-5" />
              Cadastrar atividade
            </button>
          </div>

          <Activities />
        </div>
        <div className="w-80 space-y-6">
          <ImportantLinks />
          <div className="h-px w-full bg-zinc-800" />
          <Guests />
        </div>
      </main>

      {isCreateActivityModalOpen && (
        <CreateActivityModal
          closeCreateActivityModal={() => setIsCreateActivityModalOpen(false)}
        />
      )}
    </div>
  );
}
