/** eslint-disable @typescript-eslint/no-unused-vars */
/** eslint-disable @typescript-eslint/no-unused-vars */
/** eslint-disable @typescript-eslint/no-unused-vars */
import { FormEvent, useState } from "react";
import { DateRange } from "react-day-picker";
import { useNavigate } from "react-router-dom";
import { api } from "../../lib/axios";
import { ConfirmTripModal } from "./confirm-trip-modal";
import { InviteGuestsModal } from "./invite-guests-modal";
import { DestinationAndDateStep } from "./steps/destination-and-date-step";
import { InviteGuestStep } from "./steps/invite-guests-step";

export function CreateTripPage() {
  const [isGuestInputOpen, setIsGuestInputOpen] = useState(false);
  const [isGuestModalOpen, setIsGuestModalOpen] = useState(false);
  const [isConfrimTripModalOpen, setIsConfrimTripModalOpen] = useState(false);

  const [destination, setDestination] = useState("");
  const [ownerName, setOwnerName] = useState("");
  const [ownerEmail, setOwnerEmail] = useState("");
  const [eventStartAndEndDates, setEventStartAndEndDates] = useState<
    DateRange | undefined
  >();

  const [emailsToInvite, setEmailsToInvite] = useState<string[]>([]);

  const navigate = useNavigate();

  function handleAddNewEmailToInvite(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const data = new FormData(event.currentTarget);
    const email = data.get("email")?.toString();

    if (!email) return;

    if (emailsToInvite.includes(email)) return;

    setEmailsToInvite([...emailsToInvite, email]);

    event.currentTarget.reset();
  }

  function handleRemoveEmailFromInvites(emailToRemove: string) {
    setEmailsToInvite(emailsToInvite.filter((e) => e !== emailToRemove));
  }

  async function handleCreateTrip(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!destination) {
      return;
    }

    if (
      !eventStartAndEndDates ||
      !eventStartAndEndDates.from ||
      !eventStartAndEndDates.to
    ) {
      return;
    }

    if (!ownerName) {
      return;
    }

    if (!ownerEmail) {
      return;
    }

    if (emailsToInvite.length === 0) {
      return;
    }

    console.log({
      destination,
      ownerName,
      ownerEmail,
      eventStartAndEndDates,
      emailsToInvite,
    });

    const response = await api.post("/trips", {
      destination,
      startsAt: eventStartAndEndDates.from
        .toISOString()
        .replace(/.[0-9]+Z/, ""),
      endsAt: eventStartAndEndDates.to.toISOString().replace(/.[0-9]+Z/, ""),
      ownerName,
      ownerEmail,
      emailsToInvite,
    });

    const { tripId } = response.data;

    navigate(`/trips/${tripId}`);
  }

  return (
    <div className="flex h-screen items-center justify-center bg-pattern bg-center bg-no-repeat">
      <div className="w-full max-w-3xl space-y-10 px-6 text-center">
        <div className="flex flex-col items-center gap-3">
          <img src="/logo.svg" alt="plann.er" />
          <p className="text-lg text-zinc-300">
            Convide seus amigos e planeje sua próxima viagem!
          </p>
        </div>
        <div className="space-y-4">
          <DestinationAndDateStep
            isGuestInputOpen={isGuestInputOpen}
            closeGuestInput={() => setIsGuestInputOpen(false)}
            openGuestInput={() => setIsGuestInputOpen(true)}
            onDestinationChange={(destination: string) =>
              setDestination(destination)
            }
            eventStartAndEndDates={eventStartAndEndDates}
            onStartAndEndDatesChange={(dates: DateRange | undefined) =>
              setEventStartAndEndDates(dates)
            }
          />

          {isGuestInputOpen && (
            <InviteGuestStep
              emailsToInvite={emailsToInvite}
              openGuestModal={() => setIsGuestModalOpen(true)}
              openConfirmTripModal={() => setIsConfrimTripModalOpen(true)}
            />
          )}
        </div>
        <p className="text-sm text-zinc-500">
          Ao planejar sua viagem pela plann.er você automaticamente concorda
          <br /> com nossos{" "}
          <a className="text-zinc-300 underline" href="#">
            termos de uso
          </a>{" "}
          e{" "}
          <a className="text-zinc-300 underline" href="#">
            política de privacidade
          </a>
          .
        </p>
      </div>

      {isGuestModalOpen && (
        <InviteGuestsModal
          emailsToInvite={emailsToInvite}
          handleAddNewEmailToInvite={handleAddNewEmailToInvite}
          handleRemoveEmailFromInvites={handleRemoveEmailFromInvites}
          closeGuestModal={() => setIsGuestModalOpen(false)}
        />
      )}

      {isConfrimTripModalOpen && (
        <ConfirmTripModal
          handleCreateTrip={handleCreateTrip}
          closeConfirmTripModal={() => setIsConfrimTripModalOpen(false)}
          onChangeOwnerName={(name) => setOwnerName(name)}
          onChangeOwnerEmail={(email) => setOwnerEmail(email)}
        />
      )}
    </div>
  );
}
