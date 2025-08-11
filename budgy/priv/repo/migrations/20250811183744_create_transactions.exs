defmodule Budgy.Repo.Migrations.CreateTransactions do
  use Ecto.Migration

  def change do
    create table(:transactions, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :number, :integer
      add :statement_number, :integer
      add :value_date, :utc_datetime
      add :posting_date, :utc_datetime
      add :amount, :decimal
      add :currency, :string
      add :description, :string
      add :communication, :string
      add :sender_id, references(:counterparties, on_delete: :nothing, type: :binary_id)
      add :recipient_id, references(:counterparties, on_delete: :nothing, type: :binary_id)

      timestamps(type: :utc_datetime)
    end

    create index(:transactions, [:sender_id])
    create index(:transactions, [:recipient_id])
  end
end
