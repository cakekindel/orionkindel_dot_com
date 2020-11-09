module Subheader where

import Prelude
import Utils (classes)
import Color (ribbonClass)
import Sections (Section, getColor)
import Effect (Effect)
import Effect.Aff as Aff
import Effect.Aff.Class (class MonadAff)
import Effect.Random (randomInt)
import Data.Newtype (class Newtype, unwrap)
import Data.Maybe (maybe, Maybe(..))
import Data.Array (replicate, snoc, tail, elem, length, (!!))
import Halogen as H
import Halogen.HTML as HH
import Halogen.HTML.Core as HC
import Halogen.HTML.Properties as HP
import Halogen.Query.EventSource as HES

type State = { text :: Maybe String -- text to display
             , prev :: Array String -- Array containing the last 3 randomly chosen text values
             , section :: Section
             }

data Action = ChooseNewText
            | Initialize
            | Finalize
            | SectionChanged Section

oopsText :: String
oopsText = "Oops! My code farted."

subheader =
  H.mkComponent
    { initialState
    , render
    , eval: H.mkEval $ H.defaultEval { handleAction = handleAction
                                     , initialize = Just Initialize
                                     , finalize = Just Finalize
                                     , receive = Just <<< SectionChanged
                                     }
    }
  where
    initialState section = { text: Nothing
                           , prev: [defaultText]
                           , section
                           }

render { text, section } = HH.div
      [ classes [ "app-title"
                , "flex"
                , "center-children-sec-axis"
                , "center-children-main-axis"
                ]
      ]
      [ HH.h1 [ HP.classes $ [ (getColor section # ribbonClass) ]
                             <> (HC.ClassName <$> [ "huge", "ribbon" ])
              ]
              [ HH.text $ maybe defaultText identity text ]
      ]

type EvalM o m = H.HalogenM State Action () o m Unit

handleAction :: forall o m. MonadAff m => Action -> EvalM o m
handleAction =
  let
    -- | Implementation of handleAction
    -- | shared between `Initialize` and `ChooseNewText`
    go :: forall o m. MonadAff m => Array String -> EvalM o m
    go prev = do
      -- i _think_ it's safe to not explicitly
      -- unsubscribe, since it should end with the
      -- Aff.delay
      _ <- void $ H.subscribe $ chooseNewAfterDelay
      text <- H.liftEffect $ randomText prev
      H.modify_ _ { text = pure text
                  , prev = updatePrev text prev
                  }

    -- | Updates the list of previously chosen indexes
    -- | If the list is at max length, it removes the head (oldest) and appends the new index.
    -- | If the list is below max length, it just appends the new index.
    updatePrev :: String -> Array String -> Array String
    updatePrev newIx prev | length prev >= 3 = tailOrEmpty prev `snoc` newIx
    updatePrev newIx prev                    = prev `snoc` newIx

    -- | Get the tail of an array, and return empty array
    -- | if input array was empty
    tailOrEmpty :: forall a. Array a -> Array a
    tailOrEmpty arr = case tail arr of
      Just arrTail -> arrTail
      Nothing -> []

    -- | Get random text from `textOptions`
    randomText :: Array String -> Effect String
    randomText prev = do
      ix <- randomInt 0 textOptionsMaxIx
      case textOptions !! ix of
        Just text | elem text prev -> randomText prev
        Just text                  -> pure text
        otherwise                  -> pure oopsText

    -- | A fixed-delay EventSource that will
    -- | cause the component to choose a new
    -- | `text` item from `textOptions`
    chooseNewAfterDelay :: forall m. MonadAff m => HES.EventSource m Action
    chooseNewAfterDelay =
      HES.affEventSource \emitter -> do
        Aff.delay $ Aff.Milliseconds 5000.0
        HES.emit emitter ChooseNewText
        pure mempty

    updateSection :: forall o m. MonadAff m => Section -> EvalM o m
    updateSection section = H.modify_ _ { section = section }

    init :: forall o m. MonadAff m => EvalM o m
    init = do
      _ <- H.subscribe $ chooseNewAfterDelay
      mempty
  in
    case _ of
      ChooseNewText -> H.get >>= \{prev} -> go prev
      Initialize    -> init
      Finalize      -> mempty
      SectionChanged section -> updateSection section

newtype Weight = Weight Int
derive instance newtypeWeight :: Newtype Weight _

data TextOption = TextOption Weight String

defaultText :: String
defaultText = "Orion Kindel"

textOptionsMaxIx :: Int
textOptionsMaxIx = length textOptions - 1

textOptions :: Array String
textOptions = do
  TextOption weight text <- textOptionsWeighted
  replicate (unwrap weight) text

textOptionsWeighted :: Array TextOption
textOptionsWeighted = [ TextOption (Weight 20) defaultText
                      , TextOption (Weight 15) "Always Learning"
                      , TextOption (Weight 15) "User Advocate"
                      , TextOption (Weight 15) "Beginner's Mind"
                      , TextOption (Weight 15) "Product Engineer"
                      , TextOption (Weight 15) "Master of None"
                      , TextOption (Weight 10) "Teacher"
                      , TextOption (Weight 10) "Swiss Army Knife"
                      , TextOption (Weight 10) "Student"
                      , TextOption (Weight 5 ) "Open-Source Lover"
                      , TextOption (Weight 2 ) "Glad you're here"
                      , TextOption (Weight 1 ) "Average Climber"
                      , TextOption (Weight 1 ) "<buzzwords here>"
                      ]
