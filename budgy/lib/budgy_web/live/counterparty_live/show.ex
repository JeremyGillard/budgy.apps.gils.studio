defmodule BudgyWeb.CounterpartyLive.Show do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Counterparty {@counterparty.id}
        <:subtitle>This is a counterparty record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/counterparties"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/counterparties/#{@counterparty}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit counterparty
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Name">{@counterparty.name}</:item>
        <:item title="Account">{@counterparty.account}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Show Counterparty")
     |> assign(:counterparty, Banking.get_counterparty!(id))}
  end
end
