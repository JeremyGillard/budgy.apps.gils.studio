defmodule Budgy.Banking.Address do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "addresses" do
    field :street, :string
    field :number, :string
    field :postal_code, :string
    field :city, :string
    has_many :counterparties, Budgy.Banking.Counterparty

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(address, attrs) do
    address
    |> cast(attrs, [:street, :number, :postal_code, :city])
    |> validate_required([:street, :number, :postal_code, :city])
  end
end
