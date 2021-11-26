import { styled } from "goober";

export const Form = styled("form")`
  display: flex;
  flex-direction: column;
`;

export const TextInput = styled("input")`
  display: block;
  background: #2f2f31;
  color: #fff;
  border: none;
  border-radius: 4px;
  padding: 8px 8px;
  border: 1px solid transparent;
  outline: none;
  max-width: 160px;

  &:focus {
    border-color: #0086ff;
  }
`;

export const Label = styled("label")`
  margin-bottom: 4px;
  color: #fff;
  display: block;
  font-size: 12px;
`;

export const FormItem = styled("div")`
  margin-bottom: 12px;

  &:last-child {
    margin-bottom: 0;
  }
`;

const Button = styled("button")`
  border: 1px solid transparent;
  border-radius: 4px;
  padding: 8px 8px;
  display: block;
  width: 100%;
  color: #fff;
  max-width: 160px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;

  &[disabled] {
    cursor: default;
    pointer-events: none;
    opacity: 0.4;
  }
`;

export const PrimaryButton = styled(Button)`
  background: #575759;

  &:hover {
    background: rgba(87, 87, 89, 0.8);
  }
`;

export const SubmitButton = styled(Button)`
  background: #6763ff;

  &:hover {
    background: rgba(103, 99, 255, 0.8);
  }
`;
