defmodule BudgyWeb.AddressLive.Show do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Address {@address.id}
        <:subtitle>This is a address record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/addresses"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/addresses/#{@address}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit address
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Street">{@address.street}</:item>
        <:item title="Number">{@address.number}</:item>
        <:item title="Postal code">{@address.postal_code}</:item>
        <:item title="City">{@address.city}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Show Address")
     |> assign(:address, Banking.get_address!(id))}
  end
end
