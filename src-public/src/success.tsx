import { styled } from "goober";
import React from "react";
import { PrimaryButton } from "./ui";

const Container = styled("div")`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  svg {
    width: 40px;
    margin-bottom: 12px;
  }
`;

type Props = {
  onReload(): void;
};

export function Success({ onReload }: Props) {
  return (
    <Container>
      <svg
        width="40"
        height="31"
        viewBox="0 0 372 290"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M360 11.93a40 40 0 0 0-56.57 0L122.09 193.15l-53.47-53.59c-15.619-15.635-40.955-15.649-56.59-.03-15.635 15.619-15.649 40.955-.03 56.59l80.35 80.53c.44.48.88 1 1.35 1.42a39.711 39.711 0 0 0 28.3 11.71h.09a39.761 39.761 0 0 0 28.28-11.71c.45-.46.89-.92 1.32-1.39L360 68.5c15.589-15.635 15.589-40.935 0-56.57Zm-11.31 45.26L140.24 265.51c-.11.1-.2.2-.3.31a23.842 23.842 0 0 1-17.88 7.93 23.842 23.842 0 0 1-17-7c-.31-.32-.61-.64-.91-1l-.28-.3-80.5-80.63c-9.245-9.391-9.188-24.481.128-33.802 9.316-9.322 24.405-9.388 33.802-.148l59.12 59.25a8 8 0 0 0 5.65 2.35 8 8 0 0 0 5.65-2.34l187-186.88c9.372-9.375 24.57-9.377 33.945-.005s9.377 24.57.005 33.945h.02Z"
          fill="#6763ff"
          fill-rule="nonzero"
        />
      </svg>

      <PrimaryButton onClick={onReload}>Reload</PrimaryButton>
    </Container>
  );
}
