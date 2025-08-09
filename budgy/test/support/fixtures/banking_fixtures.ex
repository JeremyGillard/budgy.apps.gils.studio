defmodule Budgy.BankingFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Budgy.Banking` context.
  """

  @doc """
  Generate a address.
  """
  def address_fixture(attrs \\ %{}) do
    {:ok, address} =
      attrs
      |> Enum.into(%{
        city: "some city",
        number: "some number",
        postal_code: "some postal_code",
        street: "some street"
      })
      |> Budgy.Banking.create_address()

    address
  end

  @doc """
  Generate a bank.
  """
  def bank_fixture(attrs \\ %{}) do
    {:ok, bank} =
      attrs
      |> Enum.into(%{
        bic: "some bic",
        country_code: "some country_code",
        name: "some name"
      })
      |> Budgy.Banking.create_bank()

    bank
  end

  @doc """
  Generate a counterparty.
  """
  def counterparty_fixture(attrs \\ %{}) do
    {:ok, counterparty} =
      attrs
      |> Enum.into(%{
        account: "some account",
        name: "some name"
      })
      |> Budgy.Banking.create_counterparty()

    counterparty
  end
end
