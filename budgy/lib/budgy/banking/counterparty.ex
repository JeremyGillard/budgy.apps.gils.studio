defmodule Budgy.Banking.Counterparty do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "counterparties" do
    field :name, :string
    has_many :account, Budgy.Banking.Account
    belongs_to :address, Budgy.Banking.Address

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(counterparty, attrs) do
    counterparty
    |> cast(attrs, [:name, :address_id])
    |> validate_required([:name])
  end
end
