defmodule Budgy.Banking.Account do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "accounts" do
    field :number, :string
    belongs_to :counterparty, Budgy.Banking.Counterparty
    belongs_to :bank, Budgy.Banking.Bank

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(account, attrs) do
    account
    |> cast(attrs, [:number, :counterparty_id, :bank_id])
    |> validate_required([:number, :counterparty_id])
  end
end
