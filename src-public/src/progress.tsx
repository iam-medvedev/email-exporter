import React from "react";
import { styled } from "goober";

type Props = {
  value: number;
};

const Container = styled("div")`
  height: 4px;
  width: 400px;
  background: #575759;
`;

const Bar = styled("div")`
  box-shadow: 1px 0px 4px 0px #3798d9;
  height: 100%;
  background: #6763ff;
`;

export function Progress({ value }: Props) {
  return (
    <Container>
      <Bar style={{ width: `${value}%` }} />
    </Container>
  );
}
