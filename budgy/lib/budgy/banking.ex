defmodule Budgy.Banking do
  @moduledoc """
  The Banking context.
  """

  import Ecto.Query, warn: false
  alias Budgy.Repo

  alias Budgy.Banking.Address

  @doc """
  Returns the list of addresses.

  ## Examples

      iex> list_addresses()
      [%Address{}, ...]

  """
  def list_addresses do
    Repo.all(Address)
  end

  @doc """
  Gets a single address.

  Raises `Ecto.NoResultsError` if the Address does not exist.

  ## Examples

      iex> get_address!(123)
      %Address{}

      iex> get_address!(456)
      ** (Ecto.NoResultsError)

  """
  def get_address!(id), do: Repo.get!(Address, id)

  @doc """
  Creates a address.

  ## Examples

      iex> create_address(%{field: value})
      {:ok, %Address{}}

      iex> create_address(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_address(attrs) do
    %Address{}
    |> Address.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a address.

  ## Examples

      iex> update_address(address, %{field: new_value})
      {:ok, %Address{}}

      iex> update_address(address, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_address(%Address{} = address, attrs) do
    address
    |> Address.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a address.

  ## Examples

      iex> delete_address(address)
      {:ok, %Address{}}

      iex> delete_address(address)
      {:error, %Ecto.Changeset{}}

  """
  def delete_address(%Address{} = address) do
    Repo.delete(address)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking address changes.

  ## Examples

      iex> change_address(address)
      %Ecto.Changeset{data: %Address{}}

  """
  def change_address(%Address{} = address, attrs \\ %{}) do
    Address.changeset(address, attrs)
  end

  alias Budgy.Banking.Bank

  @doc """
  Returns the list of banks.

  ## Examples

      iex> list_banks()
      [%Bank{}, ...]

  """
  def list_banks do
    Repo.all(Bank)
  end

  @doc """
  Gets a single bank.

  Raises `Ecto.NoResultsError` if the Bank does not exist.

  ## Examples

      iex> get_bank!(123)
      %Bank{}

      iex> get_bank!(456)
      ** (Ecto.NoResultsError)

  """
  def get_bank!(id), do: Repo.get!(Bank, id)

  @doc """
  Creates a bank.

  ## Examples

      iex> create_bank(%{field: value})
      {:ok, %Bank{}}

      iex> create_bank(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_bank(attrs) do
    %Bank{}
    |> Bank.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a bank.

  ## Examples

      iex> update_bank(bank, %{field: new_value})
      {:ok, %Bank{}}

      iex> update_bank(bank, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_bank(%Bank{} = bank, attrs) do
    bank
    |> Bank.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a bank.

  ## Examples

      iex> delete_bank(bank)
      {:ok, %Bank{}}

      iex> delete_bank(bank)
      {:error, %Ecto.Changeset{}}

  """
  def delete_bank(%Bank{} = bank) do
    Repo.delete(bank)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking bank changes.

  ## Examples

      iex> change_bank(bank)
      %Ecto.Changeset{data: %Bank{}}

  """
  def change_bank(%Bank{} = bank, attrs \\ %{}) do
    Bank.changeset(bank, attrs)
  end

  alias Budgy.Banking.Counterparty

  @doc """
  Returns the list of counterparties.

  ## Examples

      iex> list_counterparties()
      [%Counterparty{}, ...]

  """
  def list_counterparties do
    Repo.all(Counterparty)
  end

  @doc """
  Gets a single counterparty.

  Raises `Ecto.NoResultsError` if the Counterparty does not exist.

  ## Examples

      iex> get_counterparty!(123)
      %Counterparty{}

      iex> get_counterparty!(456)
      ** (Ecto.NoResultsError)

  """
  def get_counterparty!(id), do: Repo.get!(Counterparty, id)

  @doc """
  Creates a counterparty.

  ## Examples

      iex> create_counterparty(%{field: value})
      {:ok, %Counterparty{}}

      iex> create_counterparty(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_counterparty(attrs) do
    %Counterparty{}
    |> Counterparty.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a counterparty.

  ## Examples

      iex> update_counterparty(counterparty, %{field: new_value})
      {:ok, %Counterparty{}}

      iex> update_counterparty(counterparty, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_counterparty(%Counterparty{} = counterparty, attrs) do
    counterparty
    |> Counterparty.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a counterparty.

  ## Examples

      iex> delete_counterparty(counterparty)
      {:ok, %Counterparty{}}

      iex> delete_counterparty(counterparty)
      {:error, %Ecto.Changeset{}}

  """
  def delete_counterparty(%Counterparty{} = counterparty) do
    Repo.delete(counterparty)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking counterparty changes.

  ## Examples

      iex> change_counterparty(counterparty)
      %Ecto.Changeset{data: %Counterparty{}}

  """
  def change_counterparty(%Counterparty{} = counterparty, attrs \\ %{}) do
    Counterparty.changeset(counterparty, attrs)
  end

  alias Budgy.Banking.Transaction

  @doc """
  Returns the list of transactions.

  ## Examples

      iex> list_transactions()
      [%Transaction{}, ...]

  """
  def list_transactions do
    Repo.all(Transaction)
  end

  @doc """
  Gets a single transaction.

  Raises `Ecto.NoResultsError` if the Transaction does not exist.

  ## Examples

      iex> get_transaction!(123)
      %Transaction{}

      iex> get_transaction!(456)
      ** (Ecto.NoResultsError)

  """
  def get_transaction!(id), do: Repo.get!(Transaction, id)

  @doc """
  Creates a transaction.

  ## Examples

      iex> create_transaction(%{field: value})
      {:ok, %Transaction{}}

      iex> create_transaction(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_transaction(attrs) do
    %Transaction{}
    |> Transaction.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a transaction.

  ## Examples

      iex> update_transaction(transaction, %{field: new_value})
      {:ok, %Transaction{}}

      iex> update_transaction(transaction, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_transaction(%Transaction{} = transaction, attrs) do
    transaction
    |> Transaction.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a transaction.

  ## Examples

      iex> delete_transaction(transaction)
      {:ok, %Transaction{}}

      iex> delete_transaction(transaction)
      {:error, %Ecto.Changeset{}}

  """
  def delete_transaction(%Transaction{} = transaction) do
    Repo.delete(transaction)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking transaction changes.

  ## Examples

      iex> change_transaction(transaction)
      %Ecto.Changeset{data: %Transaction{}}

  """
  def change_transaction(%Transaction{} = transaction, attrs \\ %{}) do
    Transaction.changeset(transaction, attrs)
  end
end
