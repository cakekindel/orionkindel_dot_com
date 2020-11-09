module Main where

import Prelude
import Subheader (subheader)
import Utils (maybeArray, snocMaybe, classes)
import Card (CardSize(..))
import Color (bgClass)
import Sections (Section(..), getCards, getColor, allSections)
import Effect (Effect)
import Data.Symbol (SProxy(..))
import Data.Maybe (Maybe(..))
import Data.Array (snoc, (:))
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Core as HC
import Halogen.HTML.Events as HE
import Halogen.HTML.Properties as HP
import Halogen.Aff as HA
import Halogen.VDom.Driver (runUI)

main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody
  runUI component unit body

type State = Section
type Slots = { button :: forall query. H.Slot query Void Int }
data Action = ChooseSection Section

_subheader = SProxy :: SProxy "subheader"

component =
  H.mkComponent
    { initialState
    , render
    , eval: H.mkEval H.defaultEval { handleAction = handleAction }
    }
  where
    initialState _ = Present

render section =
  HH.div
    [ classes [ "app-root" ] ]
    $ (HH.slot _subheader 0 subheader section absurd)
        : [ appNavbar section ]
        <> appContent section

appNavbar selectedSection =
  HH.div
    [ classes [ "app-navbar", "flex", "center-children-sec-axis", "stretch-children-main-axis" ] ]
    (renderNavBtn selectedSection <$> allSections)
  where
    renderNavBtn selected section =
      HH.button
        [ HP.classes
            $ (getColor section # bgClass)
            : ( HC.ClassName <$> if section == selected
                                 then ["selected"]
                                 else []
              )
        , HE.onClick \_ -> if section == selected
                           then Nothing
                           else Just (ChooseSection section)
        ]
        [ HH.span [] [ navBtnText section selected ] ]

    navBtnText section selected = HH.text $ case section `compare` selected of
      EQ -> show section
      LT -> "<- " <> show section
      GT -> show section <> " ->"

    title = HH.h1
      [ classes [ "huge" ] ]
      [ HH.text "Orion Kindel" ]

appContent selectedSection =
  (renderCard <$> getCards selectedSection)
  where
    renderCard card =
      HH.div
        [ HP.classes
            $ (HC.ClassName <$> [ "card", "flex", "inline", "vert" ])
            `snocMaybe` classCardSize card
            `snoc` (getColor selectedSection # bgClass)
        ]
        [ HH.h1_ [ HH.text card.title ]
        , HH.div [ classes [ "card-content" ] ] (renderCardItem <$> card.items)
        ]

    renderCardItem i = HH.div [ classes [ "card-item" ] ]
                              [ HH.h2_ [ HH.text i.title ], i.contents ]

    classCardSize card = case card.size of
      Large -> Just $ HC.ClassName "lg"
      _     -> Nothing

handleAction = case _ of
  ChooseSection s -> H.put s
